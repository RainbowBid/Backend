use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::token_claims::TokenClaims;
use domain::entities::user::User;
use domain::interfaces::i_user_repository::IUserRepository;
use std::sync::Arc;
use tracing::error;
use tracing::log::{debug, info};

pub mod dtos {
    use fancy_regex::Regex;
    use lazy_static::lazy_static;
    use serde::Deserialize;
    use validator::{Validate, ValidationError};

    //todo!("common usage static struct - #1 to place in -> regex + validate_password");
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
