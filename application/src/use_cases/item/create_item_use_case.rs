use domain::app_error::AppError;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use anyhow::anyhow;
use domain::entities::item::Item;

pub mod dtos {
    use anyhow::anyhow;
    use domain::app_error::AppError;
    use domain::entities::item::Item;
    use domain::id::Id;
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Deserialize, Debug, Validate)]
    pub struct CreateItemRequest {
        #[validate(length(
            min = 3,
            max = 30,
            message = "Brief must be between 3 and 30 characters"
        ))]
        pub brief: String,
        #[validate(length(
            min = 3,
            max = 255,
            message = "Description must be between 3 and 255 characters"
        ))]
        pub description: String,
        pub picture: Vec<u8>,
        pub user_id: String,
    }

    impl TryFrom<CreateItemRequest> for Item {
        type Error = AppError;

        fn try_from(dto: CreateItemRequest) -> Result<Item, AppError> {
            Ok(Item::new(
                dto.brief,
                dto.description,
                dto.picture,
                Id::try_from(dto.user_id).map_err(|_| {
                    AppError::CreateItemFailed(anyhow!(
                        "Cannot assign invalid user_id to newly created item"
                    ))
                })?,
            ))
        }
    }
}

pub struct CreateItemUseCase<R: IItemRepository> {
    item_repository: Arc<R>,
}

impl<R: IItemRepository> CreateItemUseCase<R> {
    pub fn new(item_repository: Arc<R>) -> Self {
        Self { item_repository }
    }

    pub async fn execute(&self, dto: dtos::CreateItemRequest) -> Result<(), AppError> {
        let item: Item = dto.try_into()?;

        match self.item_repository.insert(item).await {
            Ok(Some(_)) => Ok(()),
            _ => Err(AppError::CreateItemFailed(anyhow!("Failed to create item"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::item::create_item_use_case::{dtos, CreateItemUseCase};
    use domain::app_error::AppError::CreateItemFailed;
    use domain::entities::item::Item;
    use domain::id::Id;
    use domain::interfaces::i_item_repository::MockIItemRepository;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn given_valid_request_when_execute_then_item_is_created() {
        // Arrange
        let mut item_repository = MockIItemRepository::new();
        let user_id = Uuid::new_v4();

        item_repository.expect_insert().returning(move |_| {
            Ok(Some(Item::new(
                "brief".to_string(),
                "description".to_string(),
                vec![0],
                Id::try_from(user_id.clone().to_string()).unwrap(),
            )))
        });

        let use_case = CreateItemUseCase::new(Arc::new(item_repository));

        let dto = dtos::CreateItemRequest {
            brief: "brief".to_string(),
            description: "description".to_string(),
            picture: vec![0],
            user_id: user_id.clone().to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_invalid_request_when_execute_then_create_item_failed_error_is_returned() {
        // Arrange
        let mut item_repository = MockIItemRepository::new();

        item_repository.expect_insert().returning(|_| Ok(None));

        let use_case = CreateItemUseCase::new(Arc::new(item_repository));

        let dto = dtos::CreateItemRequest {
            brief: "brief".to_string(),
            description: "description".to_string(),
            picture: vec![0],
            user_id: "user_id".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        match result {
            Err(CreateItemFailed(_)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }
}
