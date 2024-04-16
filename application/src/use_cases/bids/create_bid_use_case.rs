use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::auction::Auction;
use domain::entities::bid::Bid;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use std::sync::Arc;
use tracing::{error, info};

pub mod dtos {
    use anyhow::anyhow;
    use domain::app_error::AppError;
    use domain::entities::auction::Auction;
    use domain::entities::bid::Bid;
    use domain::entities::user::User;
    use domain::id::Id;
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Deserialize, Debug, Validate)]
    pub struct CreateBidRequest {
        pub value: f32,
        #[serde(skip_deserializing)]
        pub auction_id: String,
        #[serde(skip_deserializing)]
        pub user_id: String,
    }

    impl TryFrom<CreateBidRequest> for Bid {
        type Error = AppError;

        fn try_from(value: CreateBidRequest) -> Result<Bid, AppError> {
            Ok(Bid::new(
                value.value,
                Id::<Auction>::try_from(value.auction_id).map_err(|_| {
                    AppError::CreateBidFailed(anyhow!("Failed to create bid. Bad bid data."))
                })?,
                Id::<User>::try_from(value.user_id).map_err(|_| {
                    AppError::CreateBidFailed(anyhow!("Failed to create bid. Bad bid data."))
                })?,
            ))
        }
    }
}

pub struct CreateBidUseCase<R: IAuctionRepository> {
    auction_repository: Arc<R>,
}

impl<R: IAuctionRepository> CreateBidUseCase<R> {
    pub fn new(auction_repository: Arc<R>) -> Self {
        Self { auction_repository }
    }

    pub async fn execute(
        &self,
        current_user: User,
        request: dtos::CreateBidRequest,
    ) -> Result<(), AppError> {
        info!("Creating bid for auction with id: {}", request.auction_id);

        // check if user is owner of the auction
        let auction_id = Id::<Auction>::try_from(request.auction_id.clone()).map_err(|_| {
            error!(
                "Failed to get auction with auction_id = {}",
                request.auction_id
            );
            AppError::CreateBidFailed(anyhow!("Failed to create bid."))
        })?;
        let auction = self
            .auction_repository
            .find_ongoing_by_id(auction_id.clone())
            .await
            .map_err(|_| {
                error!(
                    "Failed to get auction with auction_id = {}",
                    request.auction_id
                );
                AppError::CreateBidFailed(anyhow!("Failed to create bid."))
            })?
            .ok_or_else(|| {
                error!(
                    "Failed to get auction with auction_id = {}",
                    request.auction_id
                );
                AppError::CreateBidFailed(anyhow!("Failed to create bid."))
            })?;

        if auction.end_date < chrono::Utc::now() {
            error!("Cannot bid on expired auction.");
            return Err(AppError::CannotBidOnExpiredAuction());
        }

        if request.user_id == auction.user_id.value.to_string() {
            error!(
                "Owner with id: {} cannot bid on his own auction.",
                current_user.id.to_string()
            );
            return Err(AppError::OwnerCannotBid());
        }

        let bids_result = self.auction_repository.get_all_bids(auction_id).await;

        match bids_result {
            Ok(bids) => {
                if request.value < auction.starting_price {
                    return Err(AppError::BidAmountMustBeGreaterThanStartingPrice(
                        request.value,
                        auction.starting_price,
                    ));
                }

                if let Some(highest_bid) = bids
                    .iter()
                    .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
                {
                    if request.value <= highest_bid.value {
                        return Err(AppError::BidAmountMustBeGreaterThanCurrentHighestBid(
                            request.value,
                            highest_bid.value,
                        ));
                    }
                }

                let bid = Bid::try_from(request).map_err(|_| {
                    AppError::CreateBidFailed(anyhow!("Failed to create bid. Bad bid data."))
                })?;
                match self.auction_repository.create_bid(bid).await {
                    Ok(Some(_)) => {
                        info!("Bid created successfully");
                        Ok(())
                    }
                    Ok(None) => {
                        error!("Failed to create bid: Bid not returned from repository");
                        Err(AppError::CreateBidFailedInternalServerError(anyhow!(
                            "Failed to create bid. Internal server error."
                        )))
                    }
                    Err(e) => {
                        error!("Failed when creating bid in repository: {:?}", e);
                        Err(AppError::CreateBidFailedInternalServerError(anyhow!(
                            "Failed to create bid. Internal server error."
                        )))
                    }
                }
            }
            Err(e) => {
                error!("Failed when creating bid in repository: {:?}", e);
                Err(AppError::CreateBidFailedInternalServerError(anyhow!(
                    "Failed to create bid. Internal server error."
                )))
            }
        }
    }
}
