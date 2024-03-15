use anyhow::anyhow;
use async_trait::async_trait;
use domain::app_error::AppError;
use domain::entities::item::Item;
use domain::entities::user::User;
use domain::id::Id;
use crate::models::item::ItemModel;
use crate::repositories::DatabaseRepositoryImpl;
use domain::interfaces::i_item_repository::IItemRepository;
use log::info;

#[async_trait]
impl IItemRepository for DatabaseRepositoryImpl<Item>{
    async fn get_all_by_user_id(&self, user_id: String) -> anyhow::Result<Option<Vec<Item>>> {
        let pool = self.pool.0.clone();
        let result = sqlx::query_as::<_, ItemModel>("SELECT * FROM items WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool.as_ref())
            .await
            .map_err(|e| anyhow!("{:?}", e))?;

        match result {
            _items => {
                let mut items: Vec<Item> = vec![];
                for i in _items{
                    items.push(Item::try_from(i).unwrap());
                }
                Ok(Some(items))
            },
            _ => Ok(None),
        }
    }
}