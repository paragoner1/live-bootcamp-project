// SPRINT 3: Main with app state, user store, and banned token store
// This was updated in Sprint 3 to provide complete authentication functionality

use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState,
    services::{
        hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore,
    },
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

// SPRINT 2: Main with app state and user store (commented out)
// use std::sync::Arc;
// use tokio::sync::RwLock;
//
// use auth_service::{
//     app_state::AppState, services::hashmap_user_store::HashmapUserStore, Application,
// };
//
// #[tokio::main]
// async fn main() {
//     let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
//     let app_state = AppState::new(user_store);
//
//     let app = Application::build(app_state, "0.0.0.0:3000")
//         .await
//         .expect("Failed to build app");
//
//     app.run().await.expect("Failed to run app");
// }

// SPRINT 1: Simple main without app state
// use auth_service::{app_state::AppState, Application};
// 
// #[tokio::main]
// async fn main() {
//     let app_state = AppState::new();
// 
//     let app = Application::build(app_state, "0.0.0.0:3000")
//         .await
//         .expect("Failed to build app");
// 
//     app.run().await.expect("Failed to run app");
// }