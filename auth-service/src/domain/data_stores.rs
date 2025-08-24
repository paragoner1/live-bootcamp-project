use super::{Email, Password, User};
use color_eyre::eyre::Report;
use thiserror::Error;

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

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

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        uuid::Uuid::parse_str(&id)
            .map(|_| LoginAttemptId(id))
            .map_err(|_| "Invalid UUID format".to_string())
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        LoginAttemptId(uuid::Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LoginAttemptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        if code.len() == 6 && code.chars().all(|c| c.is_ascii_digit()) {
            Ok(TwoFACode(code))
        } else {
            Err("2FA code must be exactly 6 digits".to_string())
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(100000..1000000).to_string();
        TwoFACode(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TwoFACode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_attempt_id_parse_valid_uuid() {
        let valid_uuid = "123e4567-e89b-12d3-a456-426614174000".to_string();
        let result = LoginAttemptId::parse(valid_uuid.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, valid_uuid);
    }

    #[test]
    fn test_login_attempt_id_parse_invalid_uuid() {
        let invalid_uuid = "not-a-uuid".to_string();
        let result = LoginAttemptId::parse(invalid_uuid);
        assert!(result.is_err());
    }

    #[test]
    fn test_login_attempt_id_default() {
        let id = LoginAttemptId::default();
        assert!(uuid::Uuid::parse_str(&id.0).is_ok());
    }

    #[test]
    fn test_two_fa_code_parse_valid() {
        let valid_code = "123456".to_string();
        let result = TwoFACode::parse(valid_code.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, valid_code);
    }

    #[test]
    fn test_two_fa_code_parse_invalid() {
        let invalid_codes = vec!["12345", "1234567", "abcdef", "12a456"];
        for code in invalid_codes {
            let result = TwoFACode::parse(code.to_string());
            assert!(result.is_err(), "Should fail for: {}", code);
        }
    }

    #[test]
    fn test_two_fa_code_default() {
        let code = TwoFACode::default();
        assert_eq!(code.0.len(), 6);
        assert!(code.0.chars().all(|c| c.is_ascii_digit()));
        let num: i32 = code.0.parse().unwrap();
        assert!(num >= 100000 && num < 1000000);
    }
} 