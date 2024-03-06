use crate::entities::user::User;
use crate::id::Id;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait IUserRepository {
    async fn find(&self, id: Id<User>) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: String) -> anyhow::Result<Option<User>>;
    async fn find_by_username(&self, username: String) -> anyhow::Result<Option<User>>;
    async fn insert(&self, user: User) -> anyhow::Result<Option<User>>;
}
