use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::item::Item;
use domain::entities::user::User;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use tracing::{error, info};

pub mod dtos {
    use anyhow::anyhow;
    use axum_typed_multipart::{FieldData, TryFromMultipart};
    use domain::app_error::AppError;
    use domain::entities::item::Item;
    use domain::id::Id;
    use std::io::Read;
    use tempfile::NamedTempFile;
    use validator::{Validate, ValidationError};

    fn validate_category(value: &str) -> Result<(), ValidationError> {
        match value {
            "art" | "sport" | "electronics" | "services" | "diverse" => Ok(()),
            _ => Err(ValidationError::new(
                "Invalid category. Must be one of: art, sport, electronics, services, diverse",
            )),
        }
    }

    #[derive(Debug, Validate, TryFromMultipart)]
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
        pub picture: Option<FieldData<NamedTempFile>>,
        pub user_id: Option<String>,
        #[validate(custom(
            function = "validate_category",
            message = "Invalid category. Must be one of: art, sport, electronics, services, diverse"
        ))]
        pub category: String,
    }

    impl TryFrom<CreateItemRequest> for Item {
        type Error = AppError;

        fn try_from(mut dto: CreateItemRequest) -> Result<Item, AppError> {
            let mut picture = Vec::new();
            if let Some(field) = dto.picture.as_mut() {
                field.contents.read_to_end(&mut picture).map_err(|e| {
                    AppError::CreateItemFailed(anyhow!(
                        "Failed to read picture from request: {:?}",
                        e
                    ))
                })?;
            }

            Ok(Item::new(
                dto.brief,
                dto.description,
                picture,
                Id::try_from(dto.user_id.unwrap_or_default().clone()).map_err(|_| {
                    AppError::CreateItemFailed(anyhow!(
                        "Cannot assign invalid user_id to newly created item"
                    ))
                })?,
                dto.category.into(),
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

    pub async fn execute(
        &self,
        current_user: User,
        dto: dtos::CreateItemRequest,
    ) -> Result<(), AppError> {
        info!("Creating item with brief: {}", dto.brief);

        let dto = dtos::CreateItemRequest {
            user_id: Some(current_user.id.to_string()),
            ..dto
        };
        let item: Item = dto.try_into()?;

        match self.item_repository.insert(item).await {
            Ok(Some(_)) => {
                info!("Item created successfully");
                Ok(())
            }
            _ => {
                error!("Failed to create item");
                Err(AppError::CreateItemFailed(anyhow!("Failed to create item")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::item::create_item_use_case::{dtos, CreateItemUseCase};
    use domain::app_error::AppError::CreateItemFailed;
    use domain::entities::item::{Category, Item};
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
                Category::Art,
            )))
        });

        let use_case = CreateItemUseCase::new(Arc::new(item_repository));

        let current_user = domain::entities::user::User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let dto = dtos::CreateItemRequest {
            brief: "brief".to_string(),
            description: "description".to_string(),
            picture: None,
            user_id: Some(user_id.clone().to_string()),
            category: "art".to_string(),
        };

        // Act
        let result = use_case.execute(current_user, dto).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_invalid_request_when_execute_then_create_item_failed_error_is_returned() {
        // Arrange
        let mut item_repository = MockIItemRepository::new();

        item_repository.expect_insert().returning(|_| Ok(None));

        let use_case = CreateItemUseCase::new(Arc::new(item_repository));

        let current_user = domain::entities::user::User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let dto = dtos::CreateItemRequest {
            brief: "brief".to_string(),
            description: "description".to_string(),
            picture: None,
            user_id: Some("user_id".to_string()),
            category: "art".to_string(),
        };

        // Act
        let result = use_case.execute(current_user, dto).await;

        // Assert
        match result {
            Err(CreateItemFailed(_)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }
}
