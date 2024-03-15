use crate::models::item::ItemModel;
use crate::repositories::DatabaseRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use domain::app_error::AppError;
use domain::entities::item::Item;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_item_repository::IItemRepository;
use log::{error, info};
use sqlx::types::Uuid;

#[async_trait]
impl IItemRepository for DatabaseRepositoryImpl<Item> {
    async fn get_all_by_user_id(&self, user_id: String) -> anyhow::Result<Vec<Item>> {
        let pool = self.pool.0.clone();
        let result = sqlx::query_as::<_, ItemModel>("SELECT * FROM items WHERE user_id = $1")
            .bind(Uuid::parse_str(user_id.to_string().as_str()).map_err(|e| anyhow!("{:?}", e))?)
            .fetch_all(pool.as_ref())
            .await
            .map_err(|e| {
                error!("{:?}", e);
                anyhow!("{:?}", e)
            })?;

        let items = result
            .into_iter()
            .map(|item_model| Item::try_from(item_model))
            .collect();
        items
    }
}
