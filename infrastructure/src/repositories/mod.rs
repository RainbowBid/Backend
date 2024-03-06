mod user_repository;

use crate::db::Db;
use derive_new::new;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: std::marker::PhantomData<T>,
}
