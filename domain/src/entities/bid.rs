use crate::entities::auction::Auction;
use crate::entities::user::User;
use crate::id::Id;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Bid {
    pub id: Id<Bid>,
    pub value: f32,
    pub auction_id: Id<Auction>,
    pub user_id: Id<User>,
}

impl Bid {
    pub fn new(value: f32, auction_id: Id<Auction>, user_id: Id<User>) -> Self {
        let id: Id<Bid> = Id::gen();

        Self {
            id,
            value,
            auction_id,
            user_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BidWithUsername {
    pub id: Id<Bid>,
    pub value: f32,
    pub auction_id: Id<Auction>,
    pub user_id: Id<User>,
    pub username: String,
}
