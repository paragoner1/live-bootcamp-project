// SPRINT 2: Data store traits and error types
// This was added in Sprint 2 to provide data persistence abstraction

use super::{Email, Password, User};

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// SPRINT 3: Banned token store for logout functionality
#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    TokenAlreadyBanned,
    UnexpectedError,
}

// SPRINT 1: This was the original empty implementation
// The route handlers just returned StatusCode::OK without any validation 