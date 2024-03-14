use crate::models::item::ItemModel;
use crate::repositories::DatabaseRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use domain::entities::item::Item;
use domain::interfaces::i_item_repository::IItemRepository;

#[async_trait]
impl IItemRepository for DatabaseRepositoryImpl<Item> {
    async fn insert(&self, item: Item) -> anyhow::Result<Option<Item>> {
        let pool = self.pool.0.clone();
        let item = ItemModel::try_from(item)?;
        let result = sqlx::query_as::<_, ItemModel>(
            "INSERT INTO items (id, brief, description, picture, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
            .bind(item.id)
            .bind(item.brief)
            .bind(item.description)
            .bind(item.picture)
            .bind(item.user_id)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|e| anyhow!("{:?}", e))?;

        match result {
            Some(item) => Ok(Some(Item::try_from(item)?)),
            None => Ok(None),
        }
    }
}
