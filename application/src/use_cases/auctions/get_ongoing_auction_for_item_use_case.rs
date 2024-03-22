use anyhow::anyhow;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::app_error::AppError;
use domain::entities::auction::Auction;
use domain::entities::item::Item;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use serde::Serialize;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Serialize, Debug)]
pub struct AuctionDto {
    pub id: String,
    pub item_id: String,
    pub starting_price: String,
    pub end_date: String,
}

impl IntoResponse for AuctionDto {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl AuctionDto {
    fn from_auction(auction: Auction) -> AuctionDto {
        AuctionDto {
            id: auction.id.to_string(),
            item_id: auction.item_id.to_string(),
            starting_price: auction.starting_price.to_string(),
            end_date: auction.end_date.to_string(),
        }
    }
}

pub struct GetAuctionByItemIdUseCase<R: IAuctionRepository> {
    auction_repository: Arc<R>,
}

impl<R: IAuctionRepository> GetAuctionByItemIdUseCase<R> {
    pub fn new(auction_repository: Arc<R>) -> Self {
        Self { auction_repository }
    }

    pub async fn execute(&self, item_id: String) -> Result<AuctionDto, AppError> {
        info!("Getting auction with item_id: {}", item_id);

        let parsed_item_id = Id::<Item>::try_from(item_id.clone()).map_err(|_| {
            AppError::GetItemFailed(anyhow!("Cannot get auction for invalid item_id"))
        })?;

        //todo!("check if item_id.into() is doing the parsing well : from String to Id<Item>");
        match self
            .auction_repository
            .find_ongoing_by_item_id(parsed_item_id)
            .await
        {
            Ok(Some(auction)) => {
                info!("Auction found for item_id {}", item_id);
                Ok(AuctionDto::from_auction(auction))
            }
            _ => {
                error!("Failed to get item");
                Err(AppError::GetAuctionFailed(anyhow!(
                    "Failed to get auction for item_id {}",
                    item_id
                )))
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::use_cases::items::get_item_use_case::GetItemUseCase;
//     use domain::app_error::AppError;
//     use domain::entities::item::{Category, Item};
//     use domain::interfaces::i_item_repository::MockIItemRepository;
//     use std::sync::Arc;
//
//     #[tokio::test]
//     async fn given_valid_item_id_and_user_id_when_get_item_use_case_then_return_item() {
//         //Arrange
//         let item_id = String::from("00000000-0000-0000-0000-000000000000");
//         let user_id = String::from("00000000-0000-0000-0000-000000000001");
//         let item = Item::new(
//             "brief".to_string(),
//             "description".to_string(),
//             vec![],
//             user_id.clone().try_into().unwrap(),
//             Category::Diverse,
//         );
//         let mut item_repository = MockIItemRepository::new();
//
//         let item_id_clone = item_id.clone();
//         item_repository
//             .expect_find()
//             .withf(move |id| id.value.to_string() == item_id_clone)
//             .returning(move |_| Ok(Some(item.clone())));
//
//         let item_repository = Arc::new(item_repository);
//
//         let current_user = domain::entities::user::User {
//             id: user_id.try_into().unwrap(),
//             name: "username".to_string(),
//             email: "email".to_string(),
//             password: "password".to_string(),
//         };
//         let use_case = GetItemUseCase::new(item_repository);
//
//         //Act
//         let result = use_case.execute(current_user, item_id).await;
//
//         //Assert
//         assert!(result.is_ok());
//     }
//
//     #[tokio::test]
//     async fn given_invalid_item_id_when_get_item_use_case_then_return_error() {
//         //Arrange
//         let item_id = String::from("00000000-0000-0000-0000-000000000000");
//         let mut item_repository = MockIItemRepository::new();
//
//         let item_id_clone = item_id.clone();
//         item_repository
//             .expect_find()
//             .withf(move |id| id.value.to_string() == item_id_clone)
//             .returning(|_| Ok(None));
//         let item_repository = Arc::new(item_repository);
//         let current_user = domain::entities::user::User::new(
//             "username".to_string(),
//             "email".to_string(),
//             "password".to_string(),
//         );
//         let use_case = GetItemUseCase::new(item_repository);
//
//         //Act
//         let result = use_case.execute(current_user, item_id).await;
//
//         //Assert
//         match result {
//             Err(AppError::GetItemFailed(_)) => assert!(true),
//             _ => panic!("Test failed"),
//         }
//     }
//
//     #[tokio::test]
//     async fn given_item_not_belonging_to_user_when_get_item_use_case_then_return_error() {
//         //Arrange
//         let item_id = String::from("00000000-0000-0000-0000-000000000000");
//         let item = Item::new(
//             "brief".to_string(),
//             "description".to_string(),
//             vec![],
//             String::from("00000000-0000-0000-0000-000000000001")
//                 .try_into()
//                 .unwrap(),
//             Category::Diverse,
//         );
//         let mut item_repository = MockIItemRepository::new();
//
//         let item_id_clone = item_id.clone();
//         item_repository
//             .expect_find()
//             .withf(move |id| id.value.to_string() == item_id_clone)
//             .returning(move |_| Ok(Some(item.clone())));
//         let item_repository = Arc::new(item_repository);
//         let current_user = domain::entities::user::User::new(
//             "username".to_string(),
//             "email".to_string(),
//             "password".to_string(),
//         );
//
//         let use_case = GetItemUseCase::new(item_repository);
//
//         //Act
//         let result = use_case.execute(current_user, item_id).await;
//
//         //Assert
//         match result {
//             Err(AppError::ItemDoesNotBelongToUser(_, _)) => assert!(true),
//             _ => panic!("Test failed"),
//         }
//     }
// }
