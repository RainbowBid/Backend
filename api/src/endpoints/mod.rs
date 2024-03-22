use serde::Deserialize;

pub(crate) mod auctions;
pub(crate) mod auth;
pub(crate) mod items;

#[derive(Deserialize)]
pub struct QueryFilterParamDto {
    pub category: Option<String>,
}
