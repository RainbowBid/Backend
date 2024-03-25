use crate::use_cases::auctions::handle_expired_auction_use_case::HandleExpiredAuctionUseCase;
use domain::app_error::AppError;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use domain::interfaces::i_item_repository::IItemRepository;
use futures::TryStreamExt;
use std::sync::Arc;

pub struct HandleExpiredAuctionsUseCase<R1: IAuctionRepository, R2: IItemRepository> {
    auction_repository: Arc<R1>,
    handle_expired_auction_use_case: Arc<HandleExpiredAuctionUseCase<R1, R2>>,
}

impl<R1: IAuctionRepository, R2: IItemRepository> HandleExpiredAuctionsUseCase<R1, R2> {
    pub fn new(
        auction_repository: Arc<R1>,
        handle_expired_auction_use_case: Arc<HandleExpiredAuctionUseCase<R1, R2>>,
    ) -> Self {
        Self {
            auction_repository,
            handle_expired_auction_use_case,
        }
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let expired_auctions = self
            .auction_repository
            .find_all_expired()
            .await
            .map_err(|_| {
                AppError::GetAuctionFailed("Cannot handle expired auctions".to_string())
            })?;

        expired_auctions
            .into_iter()
            .map(|auction| {
                self.handle_expired_auction_use_case
                    .execute(auction.id.value.to_string())
            })
            .collect::<futures::stream::FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        Ok(())
    }
}
