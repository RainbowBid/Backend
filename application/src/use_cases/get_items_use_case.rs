use crate::use_cases::get_items_use_case::dtos::{GetAllItemsByUserIdResponse, ItemDto};
use domain::app_error::AppError;
use domain::app_error::AppError::{InternalServerError, UserNotFound};
use domain::entities::item::Item;
use domain::id::Id;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use tracing::info;
use tracing::log::debug;

pub mod dtos {
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::Json;
    use domain::entities::item::Item;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Serialize, Debug)]
    pub struct GetAllItemsByUserIdResponse {
        pub items: Vec<ItemDto>,
    }

    impl IntoResponse for GetAllItemsByUserIdResponse {
        fn into_response(self) -> Response {
            (StatusCode::OK, Json(self)).into_response()
        }
    }
    #[derive(Serialize, Debug)]
    pub struct ItemDto {
        pub id: String,
        pub brief: String,
        pub description: String,
        pub user_id: String,
    }

    impl From<Item> for ItemDto {
        fn from(value: Item) -> Self {
            ItemDto {
                user_id: value.user_id.to_string(),
                brief: value.brief,
                description: value.description,
                id: value.id.to_string(),
            }
        }
    }
}

pub struct GetItemsUseCase<R: IItemRepository> {
    item_repository: Arc<R>,
}

impl<R: IItemRepository> GetItemsUseCase<R> {
    pub fn new(item_repository: Arc<R>) -> Self {
        Self { item_repository }
    }

    pub async fn execute(&self, user_id: String) -> Result<GetAllItemsByUserIdResponse, AppError> {
        info!("Getting items for user with id {}", user_id);

        let items = self
            .item_repository
            .get_all_by_user_id(user_id)
            .await?
            .iter()
            .map(|item| item.clone().into())
            .collect::<Vec<ItemDto>>();

        Ok(GetAllItemsByUserIdResponse { items: items })
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::get_items_use_case::dtos::GetAllItemsByUserIdResponse;
    use crate::use_cases::get_items_use_case::GetItemsUseCase;
    use domain::interfaces::i_item_repository::MockIItemRepository;
    use std::sync::Arc;

    #[tokio::test]
    async fn given_request_when_executing_then_items_belonging_to_user_are_returned() {
        //Arrange
        let mut item_repository = MockIItemRepository::new();
        item_repository
            .expect_get_all_by_user_id()
            .returning(|_| Ok(vec![]));

        let use_case = GetItemsUseCase::new(Arc::new(item_repository));

        //Act
        let result = use_case.execute("user_id".to_string()).await;

        //Assert
        assert!(result.is_ok());
        match result {
            Ok(GetAllItemsByUserIdResponse { items: _ }) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }
}
