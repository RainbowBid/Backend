use crate::db::Db;
use crate::repositories::DatabaseRepositoryImpl;
use domain::entities::user::User;
use domain::interfaces::i_repositories_module::IRepositoriesModule;
use domain::interfaces::i_user_repository::IUserRepository;
use sqlx::PgPool;

pub struct RepositoriesModule {
    pub user_repository: DatabaseRepositoryImpl<User>,
}

impl IRepositoriesModule for RepositoriesModule {
    fn user_repository(&self) -> &impl IUserRepository {
        &self.user_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: PgPool) -> Self {
        let db = Db::new(db);
        let user_repository = DatabaseRepositoryImpl::new(db);
        Self { user_repository }
    }
}
