use crate::entities::item::Item;
use crate::entities::user::User;
use crate::id::Id;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait IItemRepository {
    async fn get_all_by_user_id(&self, user_id: String) -> anyhow::Result<Vec<Item>>;
}
