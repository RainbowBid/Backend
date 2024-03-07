use domain::entities::user::User;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<UserModel> for User {
    type Error = anyhow::Error;

    fn try_from(user_table: UserModel) -> Result<Self, Self::Error> {
        Ok(User {
            id: user_table.id.to_string().try_into()?,
            name: user_table.username,
            email: user_table.email,
            password: user_table.password,
        })
    }
}

impl TryFrom<User> for UserModel {
    type Error = anyhow::Error;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        Ok(UserModel {
            id: Uuid::parse_str(&user.id.to_string())?,
            username: user.name,
            email: user.email,
            password: user.password,
        })
    }
}
