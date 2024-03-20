use crate::entities::auction::{Auction, AuctionWithItem};
use crate::entities::item::{Category, Item};
use crate::id::Id;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait IAuctionRepository {
    async fn insert(&self, auction: Auction) -> anyhow::Result<Option<Auction>>;
    async fn find_ongoing_by_item_id(&self, item_id: Id<Item>) -> anyhow::Result<Option<Auction>>;
    async fn find_all_ongoing(
        &self,
        category: Option<Category>,
    ) -> anyhow::Result<Vec<AuctionWithItem>>;
}
