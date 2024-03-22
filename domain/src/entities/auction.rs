use crate::entities::item::Item;
use crate::entities::user::User;
use crate::id::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Auction {
    pub id: Id<Auction>,
    pub item_id: Id<Item>,
    pub starting_price: f32,
    pub end_date: DateTime<Utc>,
}

impl Auction {
    pub fn new(item_id: Id<Item>, starting_price: f32, end_date: DateTime<Utc>) -> Self {
        let id = Id::gen();

        Self {
            id,
            item_id,
            starting_price,
            end_date,
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
    pub user_id: Id<User>,
}

impl AuctionWithItem {
    pub fn new(
        id: Id<Auction>,
        item_id: Id<Item>,
        starting_price: f32,
        end_date: DateTime<Utc>,
        brief: String,
        description: String,
        user_id: Id<User>,
    ) -> Self {
        Self {
            id,
            item_id,
            starting_price,
            end_date,
            brief,
            description,
            user_id,
        }
    }
}
