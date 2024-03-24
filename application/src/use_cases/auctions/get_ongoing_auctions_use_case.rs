use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::app_error::AppError;
use domain::entities::auction::AuctionWithItem;
use domain::entities::item::Category;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct GetAllDto {
    auctions: Vec<AuctionWithItemDto>,
}

impl GetAllDto {
    fn from_auctions_and_items(auctions_and_items: Vec<AuctionWithItem>) -> GetAllDto {
        GetAllDto {
            auctions: auctions_and_items
                .iter()
                .map(AuctionWithItemDto::from)
                .collect(),
        }
    }

    fn new_empty() -> GetAllDto {
        GetAllDto {
            auctions: Vec::new(),
        }
    }
}

impl IntoResponse for GetAllDto {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionWithItemDto {
    pub id: String,
    pub item_id: String,
    pub starting_price: f32,
    pub end_date: i64,
    pub brief: String,
    pub description: String,
    pub category: String,
    pub user_id: String,
}
impl AuctionWithItemDto {
    fn from(auction: &AuctionWithItem) -> Self {
        AuctionWithItemDto {
            id: auction.id.clone().to_string(),
            item_id: auction.item_id.clone().to_string(),
            starting_price: auction.starting_price,
            end_date: auction.end_date.timestamp(),
            brief: auction.brief.clone(),
            description: auction.description.clone(),
            category: auction.category.clone().into(),
            user_id: auction.user_id.clone().to_string(),
        }
    }
}

pub struct GetAuctionsUseCase<R: IAuctionRepository> {
    auction_repository: Arc<R>,
}

impl<R: IAuctionRepository> GetAuctionsUseCase<R> {
    pub fn new(auction_repository: Arc<R>) -> Self {
        Self { auction_repository }
    }

    pub async fn execute(&self, category: Option<Category>) -> Result<GetAllDto, AppError> {
        info!("Get all auctions use case start.");

        match self.auction_repository.find_all_ongoing(category).await {
            Ok(auctions_and_items_result) => {
                if auctions_and_items_result.is_empty() {
                    Ok(GetAllDto::new_empty())
                } else {
                    Ok(GetAllDto::from_auctions_and_items(
                        auctions_and_items_result,
                    ))
                }
            }
            Err(_) => {
                error!("Failed to get auctions");
                Err(AppError::FailedToGetAuctions())
            }
        }
    }
}
