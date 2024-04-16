use domain::entities::auction::{Auction, AuctionStrategy, AuctionWithItem};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct AuctionModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub starting_price: f32,
    pub end_date: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub strategy: String,
}

impl TryFrom<AuctionModel> for Auction {
    type Error = anyhow::Error;

    fn try_from(auction_table: AuctionModel) -> Result<Self, Self::Error> {
        Ok(Auction {
            id: auction_table.id.to_string().try_into()?,
            item_id: auction_table.item_id.to_string().try_into()?,
            starting_price: auction_table.starting_price,
            end_date: chrono::DateTime::from_naive_utc_and_offset(
                auction_table.end_date.naive_utc(),
                auction_table.end_date.offset().to_owned(),
            ),
            strategy: AuctionStrategy::from(auction_table.strategy),
        })
    }
}

impl TryFrom<Auction> for AuctionModel {
    type Error = anyhow::Error;

    fn try_from(auction: Auction) -> Result<Self, Self::Error> {
        Ok(AuctionModel {
            id: Uuid::parse_str(&auction.id.to_string())?,
            item_id: Uuid::parse_str(&auction.item_id.to_string())?,
            starting_price: auction.starting_price,
            end_date: sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                auction.end_date.naive_utc(),
                auction.end_date.offset().to_owned(),
            ),
            strategy: String::from(auction.strategy),
        })
    }
}

#[derive(FromRow, Debug)]
pub struct AuctionWithItemModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub starting_price: f32,
    pub end_date: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub brief: String,
    pub description: String,
    pub category: String,
    pub user_id: Uuid,
    pub strategy: String,
}

impl TryFrom<AuctionWithItemModel> for AuctionWithItem {
    type Error = anyhow::Error;

    fn try_from(auction_table: AuctionWithItemModel) -> Result<Self, Self::Error> {
        Ok(AuctionWithItem {
            id: auction_table.id.to_string().try_into()?,
            item_id: auction_table.item_id.to_string().try_into()?,
            starting_price: auction_table.starting_price,
            end_date: chrono::DateTime::from_naive_utc_and_offset(
                auction_table.end_date.naive_utc(),
                auction_table.end_date.offset().to_owned(),
            ),
            brief: auction_table.brief,
            description: auction_table.description,
            category: auction_table.category.into(),
            user_id: auction_table.user_id.to_string().try_into()?,
            strategy: AuctionStrategy::from(auction_table.strategy),
        })
    }
}

impl TryFrom<AuctionWithItem> for AuctionWithItemModel {
    type Error = anyhow::Error;

    fn try_from(auction: AuctionWithItem) -> Result<Self, Self::Error> {
        Ok(AuctionWithItemModel {
            id: Uuid::parse_str(&auction.id.to_string())?,
            item_id: Uuid::parse_str(&auction.item_id.to_string())?,
            starting_price: auction.starting_price,
            end_date: sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                auction.end_date.naive_utc(),
                auction.end_date.offset().to_owned(),
            ),
            brief: auction.brief,
            description: auction.description,
            category: auction.category.into(),
            user_id: Uuid::parse_str(&auction.user_id.to_string())?,
            strategy: String::from(auction.strategy),
        })
    }
}
