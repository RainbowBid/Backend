use crate::use_cases::items::get_items_use_case::dtos::ItemDto;
use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::item::Item;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use tracing::{error, info};

pub struct GetItemUseCase<R: IItemRepository> {
    item_repository: Arc<R>,
}

impl<R: IItemRepository> GetItemUseCase<R> {
    pub fn new(item_repository: Arc<R>) -> Self {
        Self { item_repository }
    }

    pub async fn execute(&self, current_user: User, item_id: String) -> Result<ItemDto, AppError> {
        info!("Getting item with id: {}", item_id);

        let id = Id::<Item>::try_from(item_id.clone()).map_err(|_| {
            AppError::GetItemFailed(anyhow!("Cannot assign invalid item_id to get item"))
        })?;

        match self.item_repository.find(id).await {
            Ok(Some(item)) => {
                info!("Item found");

                match item.user_id == current_user.id {
                    true => {
                        info!("Item belongs to user");
                        Ok(item.into())
                    }
                    _ => {
                        error!("Item does not belong to user");
                        Err(AppError::ItemDoesNotBelongToUser(
                            item_id,
                            current_user.id.to_string(),
                        ))
                    }
                }
            }
            _ => {
                error!("Failed to get item");
                Err(AppError::GetItemFailed(anyhow!("Failed to get item")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::items::get_item_use_case::GetItemUseCase;
    use domain::app_error::AppError;
    use domain::entities::item::{Category, Item};
    use domain::interfaces::i_item_repository::MockIItemRepository;
    use std::sync::Arc;

    #[tokio::test]
    async fn given_valid_item_id_and_user_id_when_get_item_use_case_then_return_item() {
        //Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let user_id = String::from("00000000-0000-0000-0000-000000000001");
        let item = Item::new(
            "brief".to_string(),
            "description".to_string(),
            vec![],
            user_id.clone().try_into().unwrap(),
            Category::Diverse,
        );
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(move |_| Ok(Some(item.clone())));

        let item_repository = Arc::new(item_repository);

        let current_user = domain::entities::user::User {
            id: user_id.try_into().unwrap(),
            name: "username".to_string(),
            email: "email".to_string(),
            password: "password".to_string(),
        };
        let use_case = GetItemUseCase::new(item_repository);

        //Act
        let result = use_case.execute(current_user, item_id).await;

        //Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_invalid_item_id_when_get_item_use_case_then_return_error() {
        //Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(|_| Ok(None));
        let item_repository = Arc::new(item_repository);
        let current_user = domain::entities::user::User::new(
            "username".to_string(),
            "email".to_string(),
            "password".to_string(),
        );
        let use_case = GetItemUseCase::new(item_repository);

        //Act
        let result = use_case.execute(current_user, item_id).await;

        //Assert
        match result {
            Err(AppError::GetItemFailed(_)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }

    #[tokio::test]
    async fn given_item_not_belonging_to_user_when_get_item_use_case_then_return_error() {
        //Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let item = Item::new(
            "brief".to_string(),
            "description".to_string(),
            vec![],
            String::from("00000000-0000-0000-0000-000000000001")
                .try_into()
                .unwrap(),
            Category::Diverse,
        );
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(move |_| Ok(Some(item.clone())));
        let item_repository = Arc::new(item_repository);
        let current_user = domain::entities::user::User::new(
            "username".to_string(),
            "email".to_string(),
            "password".to_string(),
        );

        let use_case = GetItemUseCase::new(item_repository);

        //Act
        let result = use_case.execute(current_user, item_id).await;

        //Assert
        match result {
            Err(AppError::ItemDoesNotBelongToUser(_, _)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }
}
