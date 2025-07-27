use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;
use yup_oauth2::{
    AccessToken, ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod,
};
use zeroize::Zeroizing;

/// Get the secure default directory for credentials and tokens
pub async fn get_secure_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("fm_data");

    // Create directory if it doesn't exist with secure permissions
    if !config_dir.exists() {
        async_fs::create_dir_all(&config_dir)
            .await
            .with_context(|| {
                format!(
                    "Failed to create config directory: {}",
                    config_dir.display()
                )
            })?;

        // Set secure permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = async_fs::metadata(&config_dir).await?.permissions();
            perms.set_mode(0o700); // rwx------ (owner only)
            async_fs::set_permissions(&config_dir, perms)
                .await
                .with_context(|| {
                    format!(
                        "Failed to set secure permissions on {}",
                        config_dir.display()
                    )
                })?;
        }
    }

    Ok(config_dir)
}

/// Check if a file has secure permissions (readable only by owner)
pub async fn check_file_permissions(file_path: &Path) -> Result<()> {
    let metadata = async_fs::metadata(file_path)
        .await
        .with_context(|| format!("Failed to read metadata for {}", file_path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let permissions = mode & 0o777;

        // File should be readable/writable by owner only (600 or stricter)
        if permissions & 0o077 != 0 {
            return Err(anyhow::anyhow!(
                "Credential file {} has insecure permissions ({:o}). Please run: chmod 600 {}",
                file_path.display(),
                permissions,
                file_path.display()
            ));
        }
    }

    #[cfg(windows)]
    {
        // On Windows, check if file is read-only for others (basic check)
        if metadata.permissions().readonly() {
            log::warn!("Credential file permissions cannot be fully verified on Windows. Ensure only you have access to: {}", file_path.display());
        }
    }

    Ok(())
}

/// Validate the structure and content of a Google OAuth credentials file
pub fn validate_credentials_content(content: &str) -> Result<()> {
    // Parse JSON to validate structure
    let json: serde_json::Value =
        serde_json::from_str(content).with_context(|| "Invalid JSON in credentials file")?;

    // Check for required OAuth2 fields
    let installed = json
        .get("installed")
        .or_else(|| json.get("web"))
        .ok_or_else(|| {
            anyhow::anyhow!("Credentials file must contain 'installed' or 'web' section")
        })?;

    let required_fields = ["client_id", "client_secret", "auth_uri", "token_uri"];
    for field in &required_fields {
        if installed.get(field).is_none() {
            return Err(anyhow::anyhow!(
                "Missing required field '{}' in credentials",
                field
            ));
        }
    }

    // Validate URLs
    let auth_uri = installed
        .get("auth_uri")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("auth_uri must be a string"))?;

    let token_uri = installed
        .get("token_uri")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("token_uri must be a string"))?;

    if !auth_uri.starts_with("https://") {
        return Err(anyhow::anyhow!("auth_uri must use HTTPS"));
    }

    if !token_uri.starts_with("https://") {
        return Err(anyhow::anyhow!("token_uri must use HTTPS"));
    }

    // Check if this looks like a Google OAuth endpoint
    if !auth_uri.contains("accounts.google.com") || !token_uri.contains("oauth2.googleapis.com") {
        log::warn!("Credentials file does not appear to be from Google - this may not work with Google Sheets API");
    }

    Ok(())
}

/// Securely read and validate credentials file
pub async fn read_application_secret_secure(credfile: &str) -> Result<ApplicationSecret> {
    let cred_path = Path::new(credfile);

    // Check file permissions
    check_file_permissions(cred_path)
        .await
        .with_context(|| "Credentials file has insecure permissions")?;

    // Read file content into secure memory and validate
    {
        let content = Zeroizing::new(
            async_fs::read_to_string(cred_path)
                .await
                .with_context(|| format!("Failed to read credentials file: {credfile}"))?,
        );

        // Validate content structure
        validate_credentials_content(&content)?;

        // content is automatically zeroed when dropped here
    }

    // Parse using yup-oauth2
    let secret = yup_oauth2::read_application_secret(credfile)
        .await
        .with_context(|| format!("Failed to parse credentials from {credfile}"))?;

    Ok(secret)
}

pub async fn create_authenticator_and_token(
    credfile: &str,
    token_cache: &str,
) -> Result<(ApplicationSecret, AccessToken)> {
    if !Path::new(credfile).exists() {
        return Err(anyhow::anyhow!(
            "Credentials file does not exist: {}",
            credfile
        ));
    }

    // Use secure credential reading with validation
    let secret = read_application_secret_secure(credfile).await?;

    // Ensure token cache directory exists and is secure
    let token_cache_path = Path::new(token_cache);
    if let Some(parent) = token_cache_path.parent() {
        if !parent.exists() {
            async_fs::create_dir_all(parent).await.with_context(|| {
                format!(
                    "Failed to create token cache directory: {}",
                    parent.display()
                )
            })?;

            // Set secure permissions on Unix systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = async_fs::metadata(parent).await?.permissions();
                perms.set_mode(0o700); // rwx------ (owner only)
                async_fs::set_permissions(parent, perms)
                    .await
                    .with_context(|| {
                        format!("Failed to set secure permissions on {}", parent.display())
                    })?;
            }
        }
    }

    let auth = InstalledFlowAuthenticator::builder(
        secret.clone(),
        InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(token_cache)
    .build()
    .await
    .with_context(|| "Failed to build authenticator")?;

    let scopes = &["https://www.googleapis.com/auth/spreadsheets"];

    let token = auth
        .token(scopes)
        .await
        .with_context(|| "Failed to obtain OAuth token")?;

    // Set secure permissions on token cache file if it was created
    if token_cache_path.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = async_fs::metadata(token_cache_path).await?.permissions();
            perms.set_mode(0o600); // rw------- (owner only)
            async_fs::set_permissions(token_cache_path, perms)
                .await
                .with_context(|| {
                    format!(
                        "Failed to set secure permissions on {}",
                        token_cache_path.display()
                    )
                })?;
        }
    }

    Ok((secret, token))
}

pub async fn read_application_secret(credfile: &str) -> Result<ApplicationSecret> {
    yup_oauth2::read_application_secret(credfile)
        .await
        .with_context(|| format!("Failed to read application secret from {credfile}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_credentials_file() -> NamedTempFile {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id",
                "project_id": "test_project",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token",
                "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
                "client_secret": "test_client_secret",
                "redirect_uris": ["http://localhost"]
            }
        }"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(credentials_json.as_bytes()).unwrap();
        temp_file
    }

    #[tokio::test]
    async fn test_read_application_secret_valid() -> Result<()> {
        let temp_file = create_test_credentials_file();
        let secret = read_application_secret(temp_file.path().to_str().unwrap()).await?;

        assert_eq!(secret.client_id, "test_client_id");
        assert_eq!(secret.client_secret, "test_client_secret");
        assert_eq!(secret.redirect_uris[0], "http://localhost");

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_credentials_content_valid() -> Result<()> {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id",
                "project_id": "test_project",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token",
                "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
                "client_secret": "test_client_secret",
                "redirect_uris": ["http://localhost"]
            }
        }"#;

        let result = validate_credentials_content(credentials_json);
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_credentials_content_missing_fields() {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id"
            }
        }"#;

        let result = validate_credentials_content(credentials_json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required field"));
    }

    #[tokio::test]
    async fn test_validate_credentials_content_insecure_urls() {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id",
                "client_secret": "test_secret",
                "auth_uri": "http://insecure.com/auth",
                "token_uri": "http://insecure.com/token"
            }
        }"#;

        let result = validate_credentials_content(credentials_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must use HTTPS"));
    }

    #[tokio::test]
    async fn test_get_secure_config_dir() -> Result<()> {
        let config_dir = get_secure_config_dir().await?;
        assert!(config_dir.ends_with("fm_data"));
        Ok(())
    }

    #[tokio::test]
    async fn test_read_application_secret_nonexistent() {
        let result = read_application_secret("/nonexistent/credentials.json").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read application secret"));
    }

    #[tokio::test]
    async fn test_read_application_secret_invalid_json() {
        let invalid_json = r#"{ "invalid": "json" }"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(invalid_json.as_bytes()).unwrap();

        let result = read_application_secret(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_authenticator_and_token_invalid_credentials() {
        let invalid_json = r#"{ "invalid": "json" }"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(invalid_json.as_bytes()).unwrap();

        let result = create_authenticator_and_token(
            temp_file.path().to_str().unwrap(),
            "test_token_cache.json",
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_authenticator_and_token_nonexistent_file() {
        let result = create_authenticator_and_token(
            "/nonexistent/credentials.json",
            "test_token_cache.json",
        )
        .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Credentials file does not exist"));
    }

    // Note: We cannot easily test successful authentication without real Google OAuth flow
    // These tests focus on input validation and error handling
}
