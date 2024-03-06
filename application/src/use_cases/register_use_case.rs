use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use domain::app_error::AppError;
use domain::entities::user::User;
use domain::interfaces::i_repositories_module::IRepositoriesModule;
use domain::interfaces::i_user_repository::IUserRepository;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

lazy_static! {
    pub static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^\da-zA-Z]).{6,}$").unwrap();
}

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    // #[validate(regex(path = "PASSWORD_REGEX"))]
    pub password: String,
}

pub struct RegisterUseCase<R: IRepositoriesModule> {
    repositories: Arc<R>,
}

impl<R: IRepositoriesModule> RegisterUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn execute(&self, dto: RegisterRequest) -> Result<(), AppError> {
        let repository = self.repositories.user_repository();

        // check whether username and email are unique
        match repository.find_by_username(dto.username.clone()).await {
            Ok(Some(_)) => return Err(AppError::UsernameAlreadyExists(dto.username.clone())),
            Err(e) => Err(e)?,
            _ => {}
        }

        match repository.find_by_email(dto.email.clone()).await {
            Ok(Some(_)) => return Err(AppError::EmailAlreadyExists(dto.email.clone())),
            Err(e) => Err(e)?,
            _ => {}
        }

        // hash password
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|_| anyhow!("Failed to hash password"))
            .map(|hash| hash.to_string())?;

        // create user account
        let user = User::new(dto.username, dto.email, hashed_password);
        repository.insert(user).await?;

        Ok(())
    }
}
