// SPRINT 3: HashmapBannedTokenStore implementation
// This was added in Sprint 3 to provide banned token storage

use std::collections::HashSet;

use crate::domain::{Token, BannedTokenStore, TokenStoreError};

#[derive(Default)]
pub struct HashmapBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashmapBannedTokenStore {
    async fn ban_token(&mut self, token: Token) -> Result<(), TokenStoreError> {
        let token_str = token.as_str().to_string();
        if self.banned_tokens.contains(&token_str) {
            return Err(TokenStoreError::TokenAlreadyBanned);
        }
        self.banned_tokens.insert(token_str);
        Ok(())
    }

    async fn is_token_banned(&self, token: &Token) -> Result<bool, TokenStoreError> {
        Ok(self.banned_tokens.contains(token.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Email, Token};

    #[tokio::test]
    async fn test_ban_token() {
        let mut token_store = HashmapBannedTokenStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let token = Token::new(&email, "secret").unwrap();

        // Test banning a new token
        let result = token_store.ban_token(token.clone()).await;
        assert!(result.is_ok());

        // Test banning the same token again
        let result = token_store.ban_token(token.clone()).await;
        assert_eq!(result, Err(TokenStoreError::TokenAlreadyBanned));
    }

    #[tokio::test]
    async fn test_is_token_banned() {
        let mut token_store = HashmapBannedTokenStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let token = Token::new(&email, "secret").unwrap();

        // Test checking unbanned token
        let result = token_store.is_token_banned(&token).await.unwrap();
        assert!(!result);

        // Test checking banned token
        token_store.ban_token(token.clone()).await.unwrap();
        let result = token_store.is_token_banned(&token).await.unwrap();
        assert!(result);
    }
} 