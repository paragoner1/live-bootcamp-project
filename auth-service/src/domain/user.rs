// SPRINT 2: User domain type implementation
// This was added in Sprint 2 to represent a user entity

use super::{Email, Password};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}

// SPRINT 1: This was the original empty implementation
// The route handlers just returned StatusCode::OK without any validation 