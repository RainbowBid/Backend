use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::app_error::AppError;
use domain::app_error::AppError::GetAuctionFailed;
use domain::entities::auction::Auction;
use domain::entities::bid::BidWithUsername;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::log::{error, info};

#[derive(Deserialize, Serialize)]
pub struct BidDto {
    pub id: String,
    pub value: f32,
    pub auction_id: String,
    pub user_id: String,
    pub username: String,
}
impl From<BidWithUsername> for BidDto {
    fn from(bid: BidWithUsername) -> Self {
        BidDto {
            id: bid.id.to_string(),
            value: bid.value,
            auction_id: bid.auction_id.to_string(),
            user_id: bid.user_id.to_string(),
            username: bid.username,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetAllDto {
    bids: Vec<BidDto>,
}

impl GetAllDto {
    fn from_auctions_and_items(bids: Vec<BidDto>) -> GetAllDto {
        GetAllDto {
            bids: bids.into_iter().map(BidDto::from).collect(),
        }
    }

    fn new_empty() -> GetAllDto {
        GetAllDto { bids: Vec::new() }
    }
}

impl IntoResponse for GetAllDto {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub struct GetBidsUseCase<R: IAuctionRepository> {
    auction_repository: Arc<R>,
}

impl<R: IAuctionRepository> GetBidsUseCase<R> {
    pub fn new(auction_repository: Arc<R>) -> Self {
        Self { auction_repository }
    }

    pub async fn execute(&self, auction_id: String) -> Result<GetAllDto, AppError> {
        info!(
            "Get all bids for auction_id = {} use case start.",
            auction_id.clone()
        );

        let auction_id = Id::<Auction>::try_from(auction_id.clone()).map_err(|_| {
            error!("Failed to parse auction_id = {}", auction_id.clone());
            GetAuctionFailed(auction_id.clone())
        })?;

        match self
            .auction_repository
            .get_all_bids(auction_id.clone())
            .await
        {
            Ok(bids_result) => {
                if bids_result.is_empty() {
                    Ok(GetAllDto::new_empty())
                } else {
                    Ok(GetAllDto::from_auctions_and_items(
                        bids_result
                            .iter()
                            .map(|bid| BidDto::from(bid.clone()))
                            .collect(),
                    ))
                }
            }
            Err(_) => {
                error!("Failed to get bids for auction with id {}", auction_id);
                Err(AppError::FailedToGetAuctions())
            }
        }
    }
}
