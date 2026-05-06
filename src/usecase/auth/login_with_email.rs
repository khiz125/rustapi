use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::AuthMethod;
use crate::domain::user::vo::Email;
use crate::usecase::auth::password_crypto::verify_password;
use crate::usecase::auth::token::issue_token;
use std::sync::Arc;

pub struct LoginWithEmailInput {
    pub email: String,
    pub password: String,
}

pub struct LoginWithEmailOutput {
    pub user_id: i64,
    pub token: String,
}

pub struct LoginWithEmailUsecase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
}

impl<R: UserRepository> LoginWithEmailUsecase<R> {
    pub fn new(user_repository: Arc<R>, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn execute(
        &self,
        input: LoginWithEmailInput,
    ) -> Result<LoginWithEmailOutput, DomainError> {
        let email = Email::new(input.email)?;

        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let password_hash = match &user.auth.auth_method {
            AuthMethod::Password { password_hash, .. } => password_hash.value(),
            AuthMethod::OAuth { .. } => return Err(DomainError::NotPasswordAuthUser),
        };

        verify_password(&input.password, password_hash)?;

        let token = issue_token(user.id.value(), &self.jwt_secret)?;

        Ok(LoginWithEmailOutput {
            user_id: user.id.value(),
            token,
        })
    }
}
