use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::auction::Auction;
use domain::entities::bid::Bid;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};
use validator::Validate;

pub mod dtos {
    use domain::app_error::AppError;
    use domain::entities::auction::Auction;
    use domain::entities::bid::Bid;
    use domain::entities::user::User;
    use domain::id::Id;
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Deserialize, Debug, Validate)]
    pub struct CreateBidRequest {
        pub id: String,
        pub value: f32,
        pub auction_id: String,
        pub user_id: String,
    }

    impl TryFrom<CreateBidRequest> for Bid {
        type Error = AppError;

        fn try_from(value: CreateBidRequest) -> Result<Bid, AppError> {
            Ok(Bid::new(
                value.value,
                Id::<Auction>::try_from(value.auction_id)?,
                Id::<User>::try_from(value.user_id)?,
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

        let auction_id: Id<Auction> =
            Id::<Auction>::try_from(request.auction_id.clone()).map_err(|_| {
                error!(
                    "Failed to get auction with auction_id = {}",
                    request.auction_id
                );
                AppError::CreateBidFailed(anyhow!("Failed to create bid."))
            })?;

        let bids_result = self.auction_repository.get_all_bids(auction_id).await;

        match bids_result {
            Ok(bids) => {
                if bids.iter().any(|bid: &Bid| bid.value > request.value) {
                    return Err(AppError::CreateBidFailed(anyhow!(
                        "New bid's value has to be the greatest from the ongoing auction."
                    )));
                }

                let bid = Bid::try_from(request).map_err(|err| AppError::CreateBidFailed(anyhow!("Failed to create bid. Bad bid data.")))?;
                match self.auction_repository.create_bid(bid).await {
                        Ok(Some(bid)) => {
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
