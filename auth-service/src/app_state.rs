// SPRINT 3: AppState with user store and banned token store
// This was updated in Sprint 3 to provide complete authentication state

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{UserStore, BannedTokenStore};

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,  // SPRINT 3: Banned token store
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedTokenStoreType) -> Self {
        Self { 
            user_store,
            banned_token_store,  // SPRINT 3: Initialize banned token store
        }
    }
}

// SPRINT 2: AppState with user store (commented out)
// use std::sync::Arc;
// use tokio::sync::RwLock;
// use crate::domain::UserStore;
// 
// pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
// 
// #[derive(Clone)]
// pub struct AppState {
//     pub user_store: UserStoreType,
// }
// 
// impl AppState {
//     pub fn new(user_store: UserStoreType) -> Self {
//         Self { user_store }
//     }
// }

// SPRINT 1: Simple placeholder - no app state needed for basic routes 