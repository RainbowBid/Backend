use domain::entities::bid::{Bid, BidWithUsername};
use sqlx::types::Uuid;
use sqlx::FromRow;
use std::convert::TryFrom;

#[derive(FromRow, Debug)]
pub struct BidModel {
    pub id: Uuid,
    pub value: f32,
    pub auction_id: Uuid,
    pub user_id: Uuid,
}

impl TryFrom<BidModel> for Bid {
    type Error = anyhow::Error;

    fn try_from(bid_table: BidModel) -> Result<Self, Self::Error> {
        Ok(Bid {
            id: bid_table.id.to_string().try_into()?,
            value: bid_table.value,
            auction_id: bid_table.auction_id.to_string().try_into()?,
            user_id: bid_table.user_id.to_string().try_into()?,
        })
    }
}

impl TryFrom<Bid> for BidModel {
    type Error = anyhow::Error;

    fn try_from(bid: Bid) -> Result<Self, Self::Error> {
        Ok(BidModel {
            id: Uuid::parse_str(&bid.id.to_string())?,
            value: bid.value,
            auction_id: Uuid::parse_str(&bid.auction_id.to_string())?,
            user_id: Uuid::parse_str(&bid.user_id.to_string())?,
        })
    }
}

#[derive(FromRow, Debug)]
pub struct BidWithUsernameModel {
    pub id: Uuid,
    pub value: f32,
    pub auction_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
}

impl TryFrom<BidWithUsernameModel> for BidWithUsername {
    type Error = anyhow::Error;

    fn try_from(bid_model: BidWithUsernameModel) -> Result<Self, Self::Error> {
        Ok(BidWithUsername {
            id: bid_model.id.to_string().try_into()?,
            value: bid_model.value,
            auction_id: bid_model.auction_id.to_string().try_into()?,
            user_id: bid_model.user_id.to_string().try_into()?,
            username: bid_model.username,
        })
    }
}
