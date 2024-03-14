use std::sync::Arc;
use tracing::info;
use domain::app_error::AppError;
use domain::app_error::AppError::{InternalServerError, UserNotFound};
use domain::entities::item::Item;
use domain::id::Id;
use domain::interfaces::i_item_repository::IItemRepository;

pub struct GetItemsUseCase<R: IItemRepository>{
    item_repository: Arc<R>,
}

impl<R: IItemRepository> GetItemsUseCase<R>{
    pub fn new(item_repository:Arc<R>) -> Self { Self {item_repository}}

    pub async fn execute(&self, user_id: String) -> Result<Vec<Item>, AppError>{
        info!("Getting items for user with id {}", user_id);

        let items = self.item_repository.get_all_by_user_id(user_id).await?;

        match  items {
            Some(items) => Ok(items),
            None => Err(InternalServerError())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use domain::entities::item::Item;
    use domain::interfaces::i_item_repository::MockIItemRepository;
    use crate::use_cases::get_items_use_case::GetItemsUseCase;

    #[tokio::test]
    async fn given_request_when_executing_then_items_belonging_to_user_are_returned(){
        //Arrange
        let mut item_repository = MockIItemRepository::new();

        item_repository
            .expect_get_all_by_user_id()
            .returning(|_| Ok(Some(Vec::new())));

        let use_case = GetItemsUseCase::new(Arc::new(item_repository));

        //Act
        let result = use_case.execute("user_id".to_string()).await;

        //Assert
        assert!(result.is_ok());
        match result {
            Ok(Some(_)) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }
}