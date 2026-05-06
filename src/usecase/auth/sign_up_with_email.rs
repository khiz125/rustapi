use crate::domain::error::DomainError;
use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::UserAuth;
use crate::domain::user::vo::{Email, UserId, UserName};
use crate::usecase::auth::password_crypto::hash_password;
use crate::usecase::auth::token::issue_token;

use std::sync::Arc;

pub struct SignUpWithEmailInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct SignUpWithEmailOutput {
    pub user_id: i64,
    pub token: String,
}

pub struct SignUpWithEmailUsecase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
}

impl<R: UserRepository> SignUpWithEmailUsecase<R> {
    pub fn new(user_repository: Arc<R>, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn execute(
        &self,
        input: SignUpWithEmailInput,
    ) -> Result<SignUpWithEmailOutput, DomainError> {
        let name = UserName::new(input.name)?;
        let email = Email::new(input.email)?;

        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(DomainError::EmailAlreadyExists(email));
        }

        let password_hash = hash_password(&input.password)?;

        let dummy_id = UserId::new(0);
        let auth = UserAuth::new_password(dummy_id, email, password_hash);
        let user = User::new(dummy_id, name, auth);

        let created = self.user_repository.create(user).await?;

        let token = issue_token(created.id.value(), &self.jwt_secret)?;

        Ok(SignUpWithEmailOutput {
            user_id: created.id.value(),
            token,
        })
    }
}
