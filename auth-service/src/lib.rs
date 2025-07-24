use std::error::Error;
use std::result::Result;
use axum::{serve::Serve, Router,};
use axum::routing::{IntoMakeService, post};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;


// This struct encapsulates our application-related logic.

pub mod routes;
pub struct Application {
    server: Serve<TcpListener, IntoMakeService<Router>, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .fallback_service(ServeDir::new("assets"))
            // Add all other routes
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token));

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server: Serve<TcpListener, IntoMakeService<Router>, Router> = axum::serve(listener, router.into_make_service());

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

// Route handlers moved to `routes` module for easier navigation
// For now we will simply return a 200 (OK) status code.

// Trigger workflow test at 12:26 PM CDT, July 24, 2025): Failed Docker username or password
// Trigger workflow test at 12:45 PM CDT, July 24, 2025): try Docker username updated reflecting case sensitive
// Trigger workflow test at 1:04 PM CDT, July 24, 2025): updated from Docker password to PAT


