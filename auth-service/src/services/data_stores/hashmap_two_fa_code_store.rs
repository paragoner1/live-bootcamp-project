use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes
            .get(email)
            .cloned()
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        // Add code
        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();

        // Get code
        let (retrieved_id, retrieved_code) = store.get_code(&email).await.unwrap();
        assert_eq!(retrieved_id, login_attempt_id);
        assert_eq!(retrieved_code, code);
    }

    #[tokio::test]
    async fn test_get_nonexistent_code() {
        let store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();

        let result = store.get_code(&email).await;
        assert!(matches!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound)));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        // Add code
        store
            .add_code(email.clone(), login_attempt_id, code)
            .await
            .unwrap();

        // Remove code
        store.remove_code(&email).await.unwrap();

        // Try to get code (should fail)
        let result = store.get_code(&email).await;
        assert!(matches!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound)));
    }
}
