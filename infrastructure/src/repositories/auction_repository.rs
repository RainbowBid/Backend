use crate::models::auction::{AuctionModel, AuctionWithItemModel};
use crate::repositories::DatabaseRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use domain::entities::auction::{Auction, AuctionWithItem};
use domain::entities::item::{Category, Item};
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use log::error;
use sqlx::types::Uuid;

#[async_trait]
impl IAuctionRepository for DatabaseRepositoryImpl<Auction> {
    async fn insert(&self, auction: Auction) -> anyhow::Result<Option<Auction>> {
        let pool = self.pool.0.clone();
        let auction = AuctionModel::try_from(auction)?;
        let result = sqlx::query_as::<_, AuctionModel>(
            "INSERT INTO auctions (id, item_id, starting_price, end_date) VALUES ($1, $2, $3, $4) RETURNING *",
        )
            .bind(auction.id)
            .bind(auction.item_id)
            .bind(auction.starting_price)
            .bind(auction.end_date)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|e| {
                error!("{:?}", e);
                anyhow!("{:?}", e)
            })?;

        match result {
            Some(auction) => Ok(Some(Auction::try_from(auction)?)),
            None => Ok(None),
        }
    }

    async fn find_ongoing_by_item_id(&self, item_id: Id<Item>) -> anyhow::Result<Option<Auction>> {
        let pool = self.pool.0.clone();
        let item_id =
            Uuid::parse_str(item_id.value.to_string().as_str()).map_err(|e| anyhow!("{:?}", e))?;

        let result = sqlx::query_as::<_, AuctionModel>(
            "SELECT * FROM auctions WHERE item_id = $1 AND end_date > now()",
        )
        .bind(item_id)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| {
            error!("{:?}", e);
            anyhow!("{:?}", e)
        })?;

        match result {
            Some(auction) => Ok(Some(Auction::try_from(auction)?)),
            None => Ok(None),
        }
    }

    async fn find_all_ongoing(
        &self,
        category: Option<Category>,
    ) -> anyhow::Result<Vec<AuctionWithItem>> {
        let pool = self.pool.0.clone();

        let category: Option<String> = category.map(|category| category.into());

        let result = sqlx::query_as::<_, AuctionWithItemModel>(
            "SELECT \
                auctions.id, \
                auctions.item_id, \
                auctions.starting_price, \
                auctions.end_date, \
                items.brief, \
                items.description, \
                items.user_id \
            FROM \
            auctions INNER JOIN items ON auctions.item_id = items.id \
            WHERE end_date > now() AND ($1 IS NULL OR items.category = $1)",
        )
        .bind(category)
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| {
            error!("{:?}", e);
            anyhow!("{:?}", e)
        })?;

        Ok(result
            .into_iter()
            .map(|auction_with_item| auction_with_item.try_into())
            .collect::<Result<Vec<AuctionWithItem>, anyhow::Error>>()?)
    }
}
