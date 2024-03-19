mod item_repository;
pub mod user_repository;

use crate::db::Db;
use sqlx::PgPool;

pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: std::marker::PhantomData<T>,
}

impl<T> DatabaseRepositoryImpl<T> {
    pub fn new(pool: PgPool) -> Self {
        let pool = Db::new(pool);

        Self {
            pool,
            _marker: std::marker::PhantomData,
        }
    }
}
