use domain::app_error::AppError;
use domain::entities::auction::Auction;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use tracing::info;

pub struct HandleExpiredAuctionUseCase<R1: IAuctionRepository, R2: IItemRepository> {
    auction_repository: Arc<R1>,
    item_repository: Arc<R2>,
}

impl<R1: IAuctionRepository, R2: IItemRepository> HandleExpiredAuctionUseCase<R1, R2> {
    pub fn new(auction_repository: Arc<R1>, item_repository: Arc<R2>) -> Self {
        Self {
            auction_repository,
            item_repository,
        }
    }

    pub async fn execute(&self, auction_id: String) -> Result<(), AppError> {
        info!("Handling expired auction with id: {}", auction_id);

        let parsed_auction_id = Id::<Auction>::try_from(auction_id.clone()).map_err(|_| {
            AppError::GetAuctionFailed(
                "Cannot handle expired auction for invalid auction_id".to_string(),
            )
        })?;

        // Get auction with item by auction_id
        let auction_with_item = self
            .auction_repository
            .find_by_id(parsed_auction_id.clone())
            .await
            .map_err(|_| {
                AppError::GetAuctionFailed(
                    "Cannot handle expired auction for invalid auction_id".to_string(),
                )
            })?
            .ok_or_else(|| AppError::NoAuctionFoundForId(parsed_auction_id.value.to_string()))?;

        // Get all bids for auction
        let bids = self
            .auction_repository
            .get_all_bids(parsed_auction_id.clone())
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
            let item_id = auction_with_item.item_id.clone();
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

        // Delete auction bids
        self.auction_repository
            .delete_bids_for_auction(parsed_auction_id.clone())
            .await
            .map_err(|_| {
                AppError::GetAuctionFailed(
                    "Cannot handle expired auction for invalid auction_id".to_string(),
                )
            })?;

        // Delete auction
        self.auction_repository
            .delete_auction(parsed_auction_id.clone())
            .await
            .map_err(|_| {
                AppError::GetAuctionFailed(
                    "Cannot handle expired auction for invalid auction_id".to_string(),
                )
            })?;

        info!(
            "Expired auction with id: {} handled successfully",
            auction_id
        );

        Ok(())
    }
}
