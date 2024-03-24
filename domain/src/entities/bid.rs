use crate::id::Id;
use crate::entities::auction::Auction;
use crate::entities::user::User;

#[derive(Debug, Clone)]
pub struct Bid {
    pub id: Id<Bid>,
    pub value: f32,
    pub auction_id: Id<Auction>,
    pub user_id: Id<User>,
}

impl Bid {
    pub fn new(id: Id<Bid>, value: i32, auction_id: Id<Auction>, user_id: Id<User>) -> Self {
        Self {
            id,
            value,
            auction_id,
            user_id,
        }
    }
}