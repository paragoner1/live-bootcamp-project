// SPRINT 3: JWT token types and functionality
// This was added in Sprint 3 to provide JWT authentication

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::Email;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user email)
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token(String);

impl Token {
    pub fn new(email: &Email, secret: &str) -> Result<Self, String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // 24 hour expiration

        let claims = Claims {
            sub: email.as_ref().to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map(Token)
        .map_err(|e| format!("Failed to create token: {}", e))
    }

    // SPRINT 3: Parse token from string for logout functionality
    pub fn parse(token_str: String) -> Result<Self, String> {
        // Basic validation that it looks like a JWT token
        if token_str.split('.').count() != 3 {
            return Err("Invalid token format".to_string());
        }
        Ok(Token(token_str))
    }

    pub fn verify(&self, secret: &str) -> Result<Claims, String> {
        decode::<Claims>(
            &self.0,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Failed to verify token: {}", e))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Email;

    #[test]
    fn test_token_creation_and_verification() {
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let secret = "test_secret";

        let token = Token::new(&email, secret).unwrap();
        let claims = token.verify(secret).unwrap();

        assert_eq!(claims.sub, "test@example.com");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_verification_with_wrong_secret() {
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let secret = "test_secret";
        let wrong_secret = "wrong_secret";

        let token = Token::new(&email, secret).unwrap();
        let result = token.verify(wrong_secret);

        assert!(result.is_err());
    }
} 