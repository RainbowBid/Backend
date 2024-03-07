use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::user::User;
use domain::interfaces::i_user_repository::IUserRepository;
use std::sync::Arc;
use tracing::{error, info};

pub mod dtos {
    use fancy_regex::Regex;
    use lazy_static::lazy_static;
    use serde::Deserialize;
    use validator::{Validate, ValidationError};

    lazy_static! {
        static ref LOWERCASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
        static ref UPPERCASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
        static ref DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
        static ref SPECIAL_REGEX: Regex = Regex::new(r"[^\da-zA-Z]").unwrap();
        static ref LENGTH_REGEX: Regex = Regex::new(r".{6,}").unwrap();
    }

    fn validate_password(value: &str) -> Result<(), ValidationError> {
        if LOWERCASE_REGEX.is_match(value).unwrap()
            && UPPERCASE_REGEX.is_match(value).unwrap()
            && DIGIT_REGEX.is_match(value).unwrap()
            && SPECIAL_REGEX.is_match(value).unwrap()
            && LENGTH_REGEX.is_match(value).unwrap()
        {
            Ok(())
        } else {
            Err(ValidationError::new(
                "Password must contain at least one lowercase letter, one uppercase letter, \
         one digit, one special character and must be at least 6 characters long",
            ))
        }
    }

    #[derive(Deserialize, Debug, Validate)]
    pub struct RegisterRequest {
        #[validate(length(
            min = 3,
            max = 30,
            message = "Username must be between 3 and 30 characters"
        ))]
        pub name: String,
        #[validate(email(message = "Invalid email"))]
        pub email: String,
        #[validate(custom(function = "validate_password",))]
        pub password: String,
    }
}

pub struct RegisterUseCase<R: IUserRepository> {
    user_repository: Arc<R>,
}

impl<R: IUserRepository> RegisterUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, dto: dtos::RegisterRequest) -> Result<(), AppError> {
        info!("Registering user with username: {}", dto.name);

        // check whether username and email are unique
        match self
            .user_repository
            .find_by_username(dto.name.clone())
            .await
        {
            Ok(Some(_)) => {
                error!("Username {} already exists", dto.name);
                return Err(AppError::UsernameAlreadyExists(dto.name.clone()));
            }
            Err(e) => {
                error!("Failed to find user by username: {:?}", e);
                Err(e)?
            }
            _ => {}
        }

        match self.user_repository.find_by_email(dto.email.clone()).await {
            Ok(Some(_)) => {
                error!("Email {} already exists", dto.email);
                return Err(AppError::EmailAlreadyExists(dto.email.clone()));
            }
            Err(e) => {
                error!("Failed to find user by email: {:?}", e);
                Err(e)?
            }
            _ => {}
        }

        // hash password
        let hashed_password = bcrypt::hash(dto.password.clone(), 12).map_err(|_| {
            error!("Failed to hash password");
            anyhow!("Failed to hash password")
        })?;

        // create user account
        let user = User::new(dto.name, dto.email, hashed_password);
        self.user_repository.insert(user).await?;
        info!("User registered successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entities::user::User;
    use domain::interfaces::i_user_repository::MockIUserRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn given_valid_dto_when_executing_then_user_is_registered() {
        // Arrange
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_find_by_username()
            .with(eq("username".to_string()))
            .returning(|_| Ok(None));
        user_repository
            .expect_find_by_email()
            .with(eq("email".to_string()))
            .returning(|_| Ok(None));
        user_repository
            .expect_insert()
            .withf(|user: &User| user.name == "username" && user.email == "email")
            .returning(|_| {
                Ok(Some(User::new(
                    "username".to_string(),
                    "email".to_string(),
                    "hashed_password".to_string(),
                )))
            });

        let use_case = RegisterUseCase::new(Arc::new(user_repository));

        let dto = dtos::RegisterRequest {
            name: "username".to_string(),
            email: "email".to_string(),
            password: "Password1!".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn giver_dto_with_existing_username_when_executing_then_username_already_exists_error_is_returned(
    ) {
        // Arrange
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_find_by_username()
            .with(eq("username".to_string()))
            .returning(|_| {
                Ok(Some(User::new(
                    "username".to_string(),
                    "email".to_string(),
                    "hashed_password".to_string(),
                )))
            });

        let use_case = RegisterUseCase::new(Arc::new(user_repository));

        let dto = dtos::RegisterRequest {
            name: "username".to_string(),
            email: "email".to_string(),
            password: "Password1!".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
        match result {
            Err(AppError::UsernameAlreadyExists(username)) => assert_eq!(username, "username"),
            _ => assert_eq!(true, false),
        }
    }

    #[tokio::test]
    async fn giver_dto_with_existing_email_when_executing_then_email_already_exists_error_is_returned(
    ) {
        // Arrange
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_find_by_username()
            .with(eq("username".to_string()))
            .returning(|_| Ok(None));
        user_repository
            .expect_find_by_email()
            .with(eq("email".to_string()))
            .returning(|_| {
                Ok(Some(User::new(
                    "username".to_string(),
                    "email".to_string(),
                    "hashed_password".to_string(),
                )))
            });

        let use_case = RegisterUseCase::new(Arc::new(user_repository));

        let dto = dtos::RegisterRequest {
            name: "username".to_string(),
            email: "email".to_string(),
            password: "Password1!".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
        match result {
            Err(AppError::EmailAlreadyExists(email)) => assert_eq!(email, "email"),
            _ => assert_eq!(true, false),
        }
    }

    #[tokio::test]
    async fn given_valid_dto_when_executing_then_user_registration_failed_error_is_returned() {
        // Arrange
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_find_by_username()
            .with(eq("username".to_string()))
            .returning(|_| Ok(None));
        user_repository
            .expect_find_by_email()
            .with(eq("email".to_string()))
            .returning(|_| Ok(None));
        user_repository
            .expect_insert()
            .withf(|user: &User| user.name == "username" && user.email == "email")
            .returning(|_| Err(anyhow!("Failed to insert user")));

        let use_case = RegisterUseCase::new(Arc::new(user_repository));

        let dto = dtos::RegisterRequest {
            name: "username".to_string(),
            email: "email".to_string(),
            password: "Password1!".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
        match result {
            Err(AppError::UserRegistrationFailed(_)) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }
}
