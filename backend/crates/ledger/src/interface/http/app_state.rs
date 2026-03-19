use std::sync::Arc;
use shared::domain::UserId;
use crate::application::ports::UserFinancesRepository;

/// Shared state injected into every Axum handler via `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub repo:    Arc<dyn UserFinancesRepository>,
    /// Fixed single-user ID until authentication is added.
    pub user_id: UserId,
}

impl AppState {
    pub fn new(repo: Arc<dyn UserFinancesRepository>, user_id: UserId) -> Self {
        Self { repo, user_id }
    }
}
