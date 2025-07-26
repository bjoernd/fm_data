use anyhow::{Context, Result};
use std::path::Path;
use yup_oauth2::{
    AccessToken, ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod,
};

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

    let secret = yup_oauth2::read_application_secret(credfile)
        .await
        .with_context(|| format!("JSON file not found: {credfile}"))?;

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
