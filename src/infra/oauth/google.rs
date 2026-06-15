use super::constants::*;
use crate::domain::error::DomainError;
use serde::Deserialize;

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

pub struct GoogleOAuthClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: reqwest::Client,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn authorization_url(&self) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline",
            GOOGLE_AUTH_URL, self.client_id, self.redirect_uri, GOOGLE_SCOPE,
        )
    }

    async fn exchange_code(&self, code: &str) -> Result<String, DomainError> {
        let response = self
            .http_client
            .post(GOOGLE_TOKEN_URL)
            .form(&[
                ("code", code),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .json::<TokenResponse>()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(response.access_token)
    }

    pub async fn fetch_user_info(&self, code: &str) -> Result<GoogleUserInfo, DomainError> {
        let access_token = self.exchange_code(code).await?;

        let user_info = self
            .http_client
            .get(GOOGLE_USER_INFO_URL)
            .bearer_auth(&access_token)
            .send()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .json::<GoogleUserInfo>()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(user_info)
    }
}
