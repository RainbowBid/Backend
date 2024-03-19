use domain::app_error::AppError;
use domain::app_error::AppError::UserNotFound;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_user_repository::IUserRepository;
use std::sync::Arc;
use tracing::{error, info};

pub struct GetUserUseCase<R: IUserRepository> {
    user_repository: Arc<R>,
}

impl<R: IUserRepository> GetUserUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: String) -> Result<User, AppError> {
        info!("Getting user with id: {}", id);

        let id = Id::try_from(id.clone()).map_err(|_| UserNotFound(id.clone()))?;
        let user = self.user_repository.find(id.clone()).await.map_err(|e| {
            error!("Failed to find user by id: {:?}", e);
            UserNotFound(id.value.to_string())
        })?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserNotFound(id.value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::interfaces::i_user_repository::MockIUserRepository;
    use mockall::predicate::eq;
    use uuid::Uuid;

    #[tokio::test]
    async fn given_valid_id_when_execute_then_return_user() {
        // Arrange
        let id = Uuid::new_v4().to_string();
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_find()
            .with(eq(Id::try_from(id.clone()).unwrap()))
            .returning(move |_| {
                Ok(Some(User::new(
                    "name".to_string(),
                    "email".to_string(),
                    "password".to_string(),
                )))
            });
        let use_case = GetUserUseCase::new(Arc::new(user_repository));

        // Act
        let result = use_case.execute(id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_id_for_non_existing_user_when_execute_then_return_user_not_found_error() {
        // Arrange
        let id = Uuid::new_v4().to_string();
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_find()
            .with(eq(Id::try_from(id.clone()).unwrap()))
            .returning(move |_| Ok(None));

        let use_case = GetUserUseCase::new(Arc::new(user_repository));

        // Act
        let result = use_case.execute(id).await;

        // Assert
        assert!(result.is_err());
    }
}
