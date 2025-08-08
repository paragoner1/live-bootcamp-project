// SPRINT 2: Data store trait implementation
// This was added in Sprint 2 to provide a data layer abstraction

use super::{Email, Password, User, Token};

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

// SPRINT 3: Banned token store for logout functionality
#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn ban_token(&mut self, token: Token) -> Result<(), TokenStoreError>;
    async fn is_token_banned(&self, token: &Token) -> Result<bool, TokenStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// SPRINT 3: Token store errors
#[derive(Debug, PartialEq)]
pub enum TokenStoreError {
    TokenAlreadyBanned,
    UnexpectedError,
}

// SPRINT 1: This was the original empty implementation
// The route handlers just returned StatusCode::OK without any validation 