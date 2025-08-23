use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = user.email.as_ref().to_string();
        let password_hash = compute_password_hash(user.password.as_ref()).await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            email,
            password_hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.as_database_error()
                .and_then(|db_err| db_err.code())
                .map(|code| code.as_ref() == "23505") // PostgreSQL unique constraint violation
                .unwrap_or(false)
            {
                UserStoreError::UserAlreadyExists
            } else {
                UserStoreError::UnexpectedError
            }
        })?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let email_str = email.as_ref().to_string();
        
        let row = sqlx::query!(
            "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1",
            email_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?
        .ok_or(UserStoreError::UserNotFound)?;

        let user = User::new(
            Email::parse(row.email).map_err(|_| UserStoreError::UnexpectedError)?,
            Password::parse(row.password_hash).map_err(|_| UserStoreError::UnexpectedError)?,
            row.requires_2fa,
        );

        Ok(user)
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let email_str = email.as_ref().to_string();
        
        let row = sqlx::query!(
            "SELECT password_hash FROM users WHERE email = $1",
            email_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?
        .ok_or(UserStoreError::UserNotFound)?;

        verify_password_hash(&row.password_hash, password.as_ref()).await
            .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }
}

// Helper function to verify if a given password matches an expected hash
// Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, this function performs hashing on a
// separate thread pool using tokio::task::spawn_blocking
async fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let expected_hash = expected_password_hash.to_string();
    let candidate = password_candidate.to_string();

    tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_hash)?;

        Argon2::default()
            .verify_password(candidate.as_bytes(), &expected_password_hash)
            .map_err(|e| e.into())
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}

// Helper function to hash passwords before persisting them in the database.
// Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, this function performs hashing on a
// separate thread pool using tokio::task::spawn_blocking
async fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let password = password.to_string();

    tokio::task::spawn_blocking(move || {
        let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

        Ok(password_hash)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}
