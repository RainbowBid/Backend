use anyhow::anyhow;
use domain::entities::user::User;
use domain::interfaces::i_repositories_module::IRepositoriesModule;
use domain::interfaces::i_user_repository::IUserRepository;
use serde::Deserialize;
use std::sync::Arc;
use lazy_static::lazy_static;
use regex::Regex;
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

    pub async fn execute(&self, dto: RegisterRequest) -> anyhow::Result<()> {
        let repository = self.repositories.user_repository();

        // check whether username and email are unique
        match repository.find_by_username(dto.username.clone()).await {
            Ok(Some(_)) => return Err(anyhow!("Username already exists")),
            Err(_) => return Err(anyhow!("User registration failed")),
            _ => {}
        }

        match repository.find_by_email(dto.email.clone()).await {
            Ok(Some(_)) => return Err(anyhow!("Email already exists")),
            Err(_) => return Err(anyhow!("User registration failed")),
            _ => {}
        }

        // create user account
        let user = User::new(dto.username, dto.email, dto.password);
        repository
            .insert(user)
            .await
            .map_err(|_| anyhow!("User registration failed"))?;

        Ok(())
    }
}
