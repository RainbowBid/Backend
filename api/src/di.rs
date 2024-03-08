use application::use_cases::login_use_case::LoginUseCase;
use application::use_cases::register_use_case::RegisterUseCase;
use domain::entities::user::User;
use infrastructure::repositories::DatabaseRepositoryImpl;
use sqlx::PgPool;
use std::sync::Arc;

pub struct Modules {
    pub(crate) register_use_case: RegisterUseCase<DatabaseRepositoryImpl<User>>,
    pub(crate) login_use_case: LoginUseCase<DatabaseRepositoryImpl<User>>,
}

impl Modules {
    pub fn new(db: PgPool) -> Self {
        let user_repository = Arc::new(DatabaseRepositoryImpl::new(db));

        let register_use_case = RegisterUseCase::new(user_repository.clone());

        let login_use_case = LoginUseCase::new(user_repository.clone());

        Self {
            register_use_case,
            login_use_case,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub modules: Arc<Modules>,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        let modules = Modules::new(db);
        let modules = Arc::new(modules);

        Self { modules }
    }
}
