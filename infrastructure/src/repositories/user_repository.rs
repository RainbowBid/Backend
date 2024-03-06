use crate::models::user::UserModel;
use crate::repositories::DatabaseRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_user_repository::IUserRepository;

#[async_trait]
impl IUserRepository for DatabaseRepositoryImpl<User> {
    async fn find(&self, id: Id<User>) -> anyhow::Result<Option<User>> {
        let pool = self.pool.0.clone();
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = $1")
            .bind(id.to_string())
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        match result {
            Some(user) => Ok(Some(User::try_from(user)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: String) -> anyhow::Result<Option<User>> {
        let pool = self.pool.0.clone();
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        match result {
            Some(user) => Ok(Some(User::try_from(user)?)),
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: String) -> anyhow::Result<Option<User>> {
        let pool = self.pool.0.clone();
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        match result {
            Some(user) => Ok(Some(User::try_from(user)?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, user: User) -> anyhow::Result<Option<User>> {
        let pool = self.pool.0.clone();
        let user = UserModel::try_from(user)?;
        let result = sqlx::query_as::<_, UserModel>(
            "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|err| anyhow!("{:?}", err))?;

        match result {
            Some(user) => Ok(Some(User::try_from(user)?)),
            None => Ok(None),
        }
    }
}
