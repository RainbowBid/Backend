use crate::entities::auction::{Auction, AuctionWithItem};
use crate::entities::bid::{Bid, BidWithUsername};
use crate::entities::item::{Category, Item};
use crate::id::Id;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait IAuctionRepository {
    async fn insert(&self, auction: Auction) -> anyhow::Result<Option<Auction>>;
    async fn find_all_expired(&self) -> anyhow::Result<Vec<Auction>>;
    async fn find_by_id(&self, auction_id: Id<Auction>) -> anyhow::Result<Option<AuctionWithItem>>;
    async fn find_ongoing_by_item_id(&self, item_id: Id<Item>) -> anyhow::Result<Option<Auction>>;
    async fn find_ongoing_by_id(
        &self,
        auction_id: Id<Auction>,
    ) -> anyhow::Result<Option<AuctionWithItem>>;
    async fn find_all_ongoing(
        &self,
        category: Option<Category>,
    ) -> anyhow::Result<Vec<AuctionWithItem>>;

    async fn create_bid(&self, bid: Bid) -> anyhow::Result<Option<Bid>>;
    async fn get_all_bids(&self, auction_id: Id<Auction>) -> anyhow::Result<Vec<BidWithUsername>>;

    async fn delete_auction(&self, auction_id: Id<Auction>) -> anyhow::Result<()>;

    async fn delete_bids_for_auction(&self, auction_id: Id<Auction>) -> anyhow::Result<()>;
}
