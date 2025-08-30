use super::{Email, Password, User};
use color_eyre::eyre::{eyre, Context, Report, Result};
use secrecy::{ExposeSecret, Secret};
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
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Token already banned")]
    TokenAlreadyBanned,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
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

#[derive(Debug, Error)]
pub enum TwoFACodeStoreError {
    #[error("Login Attempt ID not found")]
    LoginAttemptIdNotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Debug, Clone)]
pub struct LoginAttemptId(Secret<String>);

impl PartialEq for LoginAttemptId {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl LoginAttemptId {
    pub fn parse(id: Secret<String>) -> Result<Self> {
        let parsed_id = uuid::Uuid::parse_str(id.expose_secret()).wrap_err("Invalid login attempt id")?;
        Ok(Self(parsed_id.to_string().into()))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        LoginAttemptId(Secret::new(uuid::Uuid::new_v4().to_string()))
    }
}

impl AsRef<Secret<String>> for LoginAttemptId {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl std::fmt::Display for LoginAttemptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.expose_secret())
    }
}

#[derive(Clone, Debug)]
pub struct TwoFACode(Secret<String>);

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl TwoFACode {
    pub fn parse(code: Secret<String>) -> Result<Self> {
        let code_as_u32 = code.expose_secret().parse::<u32>().wrap_err("Invalid 2FA code")?;

        if (100_000..=999_999).contains(&code_as_u32) {
            Ok(Self(code))
        } else {
            Err(eyre!("Invalid 2FA code"))
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
        TwoFACode(Secret::new(code))
    }
}

impl AsRef<Secret<String>> for TwoFACode {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl std::fmt::Display for TwoFACode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.expose_secret())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_attempt_id_parse_valid_uuid() {
        let valid_uuid = "123e4567-e89b-12d3-a456-426614174000".to_string();
        let result = LoginAttemptId::parse(Secret::new(valid_uuid.clone()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.expose_secret(), &valid_uuid);
    }

    #[test]
    fn test_login_attempt_id_parse_invalid_uuid() {
        let invalid_uuid = "not-a-uuid".to_string();
        let result = LoginAttemptId::parse(Secret::new(invalid_uuid));
        assert!(result.is_err());
    }

    #[test]
    fn test_login_attempt_id_default() {
        let id = LoginAttemptId::default();
        assert!(uuid::Uuid::parse_str(id.0.expose_secret()).is_ok());
    }

    #[test]
    fn test_two_fa_code_parse_valid() {
        let valid_code = "123456".to_string();
        let result = TwoFACode::parse(Secret::new(valid_code.clone()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.expose_secret(), &valid_code);
    }

    #[test]
    fn test_two_fa_code_parse_invalid() {
        let invalid_codes = vec!["12345", "1234567", "abcdef", "12a456"];
        for code in invalid_codes {
            let result = TwoFACode::parse(Secret::new(code.to_string()));
            assert!(result.is_err(), "Should fail for: {}", code);
        }
    }

    #[test]
    fn test_two_fa_code_default() {
        let code = TwoFACode::default();
        assert_eq!(code.0.expose_secret().len(), 6);
        assert!(code.0.expose_secret().chars().all(|c| c.is_ascii_digit()));
        let num: i32 = code.0.expose_secret().parse().unwrap();
        assert!(num >= 100000 && num < 1000000);
    }
} 