use crate::models::item::ItemModel;
use crate::repositories::DatabaseRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use domain::entities::item::{Category, Item};
use domain::id::Id;
use domain::interfaces::i_item_repository::IItemRepository;
use log::{error, info};
use sqlx::types::Uuid;

#[async_trait]
impl IItemRepository for DatabaseRepositoryImpl<Item> {
    async fn insert(&self, item: Item) -> anyhow::Result<Option<Item>> {
        let pool = self.pool.0.clone();
        let item = ItemModel::try_from(item)?;
        let result = sqlx::query_as::<_, ItemModel>(
            "INSERT INTO items (id, brief, description, picture, user_id, category) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
            .bind(item.id)
            .bind(item.brief)
            .bind(item.description)
            .bind(item.picture)
            .bind(item.user_id)
            .bind(item.category)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|e| anyhow!("{:?}", e))?;

        match result {
            Some(item) => Ok(Some(Item::try_from(item)?)),
            None => Ok(None),
        }
    }
    async fn find(&self, id: Id<Item>) -> anyhow::Result<Option<Item>> {
        let pool = self.pool.0.clone();
        let id = Uuid::parse_str(id.value.to_string().as_str()).map_err(|e| anyhow!("{:?}", e))?;

        let result = sqlx::query_as::<_, ItemModel>("SELECT * FROM items WHERE id = $1")
            .bind(id)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|e| anyhow!("{:?}", e))?;

        match result {
            Some(item) => Ok(Some(Item::try_from(item)?)),
            None => Ok(None),
        }
    }

    async fn get_all_by_user_id(
        &self,
        user_id: String,
        category: Option<Category>,
    ) -> anyhow::Result<Vec<Item>> {
        let pool = self.pool.0.clone();

        let category: Option<String> = category.map(|category| category.into());
        info!("{:?}", category);

        let result = sqlx::query_as::<_, ItemModel>(
            "SELECT * FROM items WHERE user_id = $1 AND (category = $2 OR $2 IS NULL)",
        )
        .bind(Uuid::parse_str(user_id.to_string().as_str()).map_err(|e| anyhow!("{:?}", e))?)
        .bind(category)
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| {
            error!("{:?}", e);
            anyhow!("{:?}", e)
        })?;

        result.into_iter().map(Item::try_from).collect()
    }
}
