use anyhow::{Context, Result};
use std::path::Path;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod, ApplicationSecret, AccessToken};

pub async fn create_authenticator_and_token(credfile: &str, token_cache: &str) -> Result<(ApplicationSecret, AccessToken)> {
    if !Path::new(credfile).exists() {
        return Err(anyhow::anyhow!("Credentials file does not exist: {}", credfile));
    }

    let secret = yup_oauth2::read_application_secret(credfile)
        .await
        .with_context(|| format!("JSON file not found: {}", credfile))?;

    let auth = InstalledFlowAuthenticator::builder(
        secret.clone(),
        InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(token_cache)
    .build()
    .await
    .with_context(|| "Failed to build authenticator")?;

    let scopes = &["https://www.googleapis.com/auth/spreadsheets"];
    
    let token = auth.token(scopes)
        .await
        .with_context(|| "Failed to obtain OAuth token")?;

    Ok((secret, token))
}

pub async fn read_application_secret(credfile: &str) -> Result<ApplicationSecret> {
    yup_oauth2::read_application_secret(credfile)
        .await
        .with_context(|| format!("Failed to read application secret from {}", credfile))
}