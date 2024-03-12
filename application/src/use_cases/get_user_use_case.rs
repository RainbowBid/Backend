use domain::app_error::AppError;
use domain::app_error::AppError::UserNotFound;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_user_repository::IUserRepository;
use std::sync::Arc;
use tracing::info;

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
        let user = self.user_repository.find(id.clone()).await?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserNotFound(id.value.to_string())),
        }
    }
}
