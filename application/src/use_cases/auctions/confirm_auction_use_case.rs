use domain::app_error::AppError;
use domain::entities::auction::{Auction, AuctionStrategy};
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use domain::interfaces::i_item_repository::IItemRepository;
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct AuctionConfirmationRequest {
    pub is_confirmed: bool,
}

pub struct ConfirmAuctionUseCase<R1: IAuctionRepository, R2: IItemRepository> {
    auction_repository: Arc<R1>,
    item_repository: Arc<R2>,
}

impl<R1: IAuctionRepository, R2: IItemRepository> ConfirmAuctionUseCase<R1, R2> {
    pub fn new(auction_repository: Arc<R1>, item_repository: Arc<R2>) -> Self {
        Self {
            auction_repository,
            item_repository,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        auction_id: String,
        request: AuctionConfirmationRequest,
    ) -> Result<(), AppError> {
        // check if user is owner of auction
        let auction_id = Id::<Auction>::try_from(auction_id.clone()).map_err(|_| {
            error!("Failed to get auction with auction_id = {}", auction_id);
            AppError::AuctionConfirmationFailed()
        })?;

        let auction = self
            .auction_repository
            .find_by_id(auction_id.clone())
            .await
            .map_err(|_| {
                error!("Failed to get auction with auction_id = {}", auction_id);
                AppError::AuctionConfirmationFailed()
            })?
            .ok_or_else(|| {
                error!("Failed to get auction with auction_id = {}", auction_id);
                AppError::AuctionConfirmationFailed()
            })?;

        if auction.end_date > chrono::Utc::now() {
            error!("Cannot confirm auction if auction is not expired");
            return Err(AppError::CannotConfirmAuctionIfAuctionIsNotExpired());
        }

        if auction.strategy != AuctionStrategy::RequestFinalApproval {
            error!("Cannot confirm auction if strategy is not RequestFinalApproval");
            return Err(AppError::CannotConfirmAuctionIfStrategyIsNotRequestFinalApproval());
        }

        if current_user.id.value != auction.user_id.value {
            error!("Only owner of this auction can confirm it");
            return Err(AppError::CannotConfirmAuctionIfUserIsNotOwner());
        }

        match request.is_confirmed {
            true => {
                // Get all bids for auction
                let bids = self
                    .auction_repository
                    .get_all_bids(auction_id.clone())
                    .await
                    .map_err(|_| {
                        AppError::GetAuctionFailed(
                            "Cannot handle expired auction for invalid auction_id".to_string(),
                        )
                    })?;

                // If any bids, get the highest bid and update item with the new owner taken from the highest bid
                let highest_bid = bids
                    .iter()
                    .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

                if let Some(highest_bid) = highest_bid {
                    let item_id = auction.item_id.clone();
                    let new_owner_id = highest_bid.user_id.clone();

                    self.item_repository
                        .change_owner(item_id, new_owner_id)
                        .await
                        .map_err(|_| {
                            AppError::GetAuctionFailed(
                                "Cannot handle expired auction for invalid auction_id".to_string(),
                            )
                        })?;
                }
            }
            false => {}
        }

        // Delete auction bids
        self.auction_repository
            .delete_bids_for_auction(auction_id.clone())
            .await
            .map_err(|_| {
                AppError::GetAuctionFailed(
                    "Cannot handle expired auction for invalid auction_id".to_string(),
                )
            })?;

        // Delete auction
        self.auction_repository
            .delete_auction(auction_id.clone())
            .await
            .map_err(|_| {
                AppError::GetAuctionFailed(
                    "Cannot handle expired auction for invalid auction_id".to_string(),
                )
            })?;

        Ok(())
    }
}
