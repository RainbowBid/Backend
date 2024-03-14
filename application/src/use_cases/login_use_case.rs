use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::user::User;
use domain::interfaces::i_user_repository::IUserRepository;
use std::sync::Arc;
use tracing::error;
use tracing::log::info;

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

    #[derive(Deserialize, Debug, Validate, Clone)]
    pub struct LoginRequest {
        #[validate(email(message = "Invalid email"))]
        pub email: String,
        #[validate(custom(function = "validate_password",))]
        pub password: String,
    }
}

pub struct LoginUseCase<R: IUserRepository> {
    user_repository: Arc<R>,
}

impl<R: IUserRepository> LoginUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, dto: dtos::LoginRequest) -> Result<User, AppError> {
        info!("Logging user with email: {}", dto.email);

        // check wheather the email is registered AND the password matches the hash.
        let user: User = match self.user_repository.find_by_email(dto.email.clone()).await {
            Ok(Some(user)) => user,
            Err(e) => {
                error!("Failed to find user by email: {:?}", e);
                Err(e)?
            }
            _ => {
                error!("Email {} is not registered.", dto.email);
                return Err(AppError::NotRegisteredEmail(dto.email.clone()));
            }
        };

        match bcrypt::verify(dto.password.clone(), user.password.as_str()).map_err(|_| {
            error!("Failed to hash password");
            anyhow!("Failed to hash password")
        })? {
            true => {
                info!("Login succeeded!");
            }
            false => {
                error!("Bad password.");
                return Err(AppError::BadPassword());
            }
        }

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::login_use_case::{dtos, LoginUseCase};
    use anyhow::anyhow;
    use domain::app_error::AppError;
    use domain::entities::user::User;
    use domain::interfaces::i_user_repository::MockIUserRepository;
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn given_request_with_valid_data_when_executing_then_user_is_signed_in() {
        //Arrange
        let mut user_repository = MockIUserRepository::new();
        let user = User::new(
            "name".to_string(),
            "email".to_string(),
            bcrypt::hash("password".to_string(), 12).unwrap(),
        );
        user_repository
            .expect_find_by_email()
            .with(eq("email".to_string()))
            .returning(move |_| Ok(Some(user.clone())));

        let use_case = LoginUseCase::new(Arc::new(user_repository));
        let dto = dtos::LoginRequest {
            email: "email".to_string(),
            password: "password".to_string(),
        };

        //Act
        let result = use_case.execute(dto).await;

        //Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_request_with_invalid_email_when_executing_then_not_registered_email_error_is_returned(
    ) {
        //Arrange
        let mut user_repository = MockIUserRepository::new();
        user_repository
            .expect_find_by_email()
            .returning(|_| Ok(None));

        let use_case = LoginUseCase::new(Arc::new(user_repository));
        let dto = dtos::LoginRequest {
            email: "invalid_email".to_string(),
            password: "password".to_string(),
        };

        //Act
        let result = use_case.execute(dto.clone()).await;

        //Assert
        assert!(result.is_err());
        match result {
            Err(AppError::NotRegisteredEmail(message)) => assert_eq!(message, dto.email.clone()),
            _ => assert_eq!(true, false),
        }
    }

    #[tokio::test]
    async fn given_request_with_bad_password_when_executing_then_bad_password_error_is_returned() {
        //Arrange
        let mut user_repository = MockIUserRepository::new();
        user_repository.expect_find_by_email().returning(|_| {
            Ok(Some(User::new(
                "name".to_string(),
                "email".to_string(),
                bcrypt::hash("password".to_string(), 12).unwrap(),
            )))
        });

        let use_case = LoginUseCase::new(Arc::new(user_repository));
        let dto = dtos::LoginRequest {
            email: "email".to_string(),
            password: "another_password".to_string(),
        };

        //Act
        let result = use_case.execute(dto.clone()).await;

        //Assert
        assert!(result.is_err());
        match result {
            Err(AppError::BadPassword()) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }

    #[tokio::test]
    async fn given_any_request_when_executing_then_unexpected_error_is_returned() {
        //Arrange
        let mut user_repository = MockIUserRepository::new();
        user_repository
            .expect_find_by_email()
            .returning(|_| Err(anyhow!("Unexpected_error")));

        let use_case = LoginUseCase::new(Arc::new(user_repository));
        let dto = dtos::LoginRequest {
            email: "email".to_string(),
            password: "password".to_string(),
        };

        //Act
        let result = use_case.execute(dto.clone()).await;

        //Assert
        assert!(result.is_err());
        match result {
            Err(_) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }
}
