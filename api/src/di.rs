use application::use_cases::register_use_case::RegisterUseCase;
use infrastructure::modules::RepositoriesModule;
use sqlx::PgPool;
use std::sync::Arc;

pub struct Modules {
    pub(crate) register_use_case: RegisterUseCase<RepositoriesModule>,
}

impl Modules {
    pub fn new(db: PgPool) -> Self {
        let repositories_module = RepositoriesModule::new(db);

        let register_use_case = RegisterUseCase::new(Arc::new(repositories_module));

        Self { register_use_case }
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
