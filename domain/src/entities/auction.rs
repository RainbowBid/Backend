use crate::entities::item::{Category, Item};
use crate::entities::user::User;
use crate::id::Id;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Auction {
    pub id: Id<Auction>,
    pub item_id: Id<Item>,
    pub starting_price: f32,
    pub end_date: DateTime<Utc>,
    pub strategy: AuctionStrategy,
}

impl Auction {
    pub fn new(
        item_id: Id<Item>,
        starting_price: f32,
        end_date: DateTime<Utc>,
        strategy: AuctionStrategy,
    ) -> Self {
        let id = Id::gen();

        Self {
            id,
            item_id,
            starting_price,
            end_date,
            strategy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuctionWithItem {
    pub id: Id<Auction>,
    pub item_id: Id<Item>,
    pub starting_price: f32,
    pub end_date: DateTime<Utc>,
    pub brief: String,
    pub description: String,
    pub category: Category,
    pub user_id: Id<User>,
    pub strategy: AuctionStrategy,
}

impl AuctionWithItem {
    pub fn new(
        id: Id<Auction>,
        item_id: Id<Item>,
        starting_price: f32,
        end_date: DateTime<Utc>,
        brief: String,
        description: String,
        category: Category,
        user_id: Id<User>,
        strategy: AuctionStrategy,
    ) -> Self {
        Self {
            id,
            item_id,
            starting_price,
            end_date,
            brief,
            description,
            category,
            user_id,
            strategy,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AuctionStrategy {
    Standard,
    RequestFinalApproval,
}

impl From<String> for AuctionStrategy {
    fn from(strategy: String) -> Self {
        match strategy.as_str() {
            "standard" => Self::Standard,
            "request_final_approval" => Self::RequestFinalApproval,
            _ => Self::Standard,
        }
    }
}

impl From<AuctionStrategy> for String {
    fn from(strategy: AuctionStrategy) -> Self {
        match strategy {
            AuctionStrategy::Standard => "standard".to_string(),
            AuctionStrategy::RequestFinalApproval => "request_final_approval".to_string(),
        }
    }
}
