use std::error::Error;

// SPRINT 2: Complete application with error handling
// This was added in Sprint 2 to provide proper error handling and response mapping

use app_state::AppState;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use routes::{login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

// Re-export services for easier access
pub use services::hashmap_user_store::HashmapUserStore;
pub use services::hashmap_banned_token_store::HashmapBannedTokenStore;  // SPRINT 3: Banned token store

// SPRINT 1: Basic application structure (commented out)
// use axum::{
//     routing::post,
//     serve::Serve,
//     Router,
// };
// use tower_http::services::ServeDir;
// 
// pub mod app_state;
// pub mod domain;  // Add this back so we can test the domain types
// pub mod routes;
// pub mod services;  // Add this so HashmapUserStore tests can run
// 
// pub struct Application {
//     server: Serve<Router, Router>,
//     pub address: String,
// }
// 
// impl Application {
//     pub async fn build(app_state: app_state::AppState, address: &str) -> Result<Self, Box<dyn Error>> {
//         let router = Router::new()
//             .nest_service("/", ServeDir::new("assets"))
//             .route("/signup", post(routes::signup))
//             .route("/login", post(routes::login))
//             .route("/verify-2fa", post(routes::verify_2fa))
//             .route("/logout", post(routes::logout))
//             .route("/verify-token", post(routes::verify_token))
//             .with_state(app_state);
// 
//         let listener = tokio::net::TcpListener::bind(address).await?;
//         let address = listener.local_addr()?.to_string();
//         let server = axum::serve(listener, router);
// 
//         Ok(Application { server, address })
//     }
// 
//     pub async fn run(self) -> Result<(), std::io::Error> {
//         println!("listening on {}", &self.address);
//         self.server.await
//     }
// }
// 
// // Re-export services for easier access
// pub use services::hashmap_user_store::HashmapUserStore;


