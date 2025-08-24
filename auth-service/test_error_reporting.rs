use color_eyre::eyre::{eyre, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("Database connection failed")]
    DatabaseError(#[source] eyre::Report),
    #[error("Authentication failed")]
    AuthError(#[source] eyre::Report),
}

fn simulate_database_error() -> Result<(), TestError> {
    // Simulate a database connection error
    Err(TestError::DatabaseError(eyre!("Connection timeout after 30 seconds")))
}

fn simulate_auth_error() -> Result<(), TestError> {
    // Simulate an authentication error
    Err(TestError::AuthError(eyre!("Invalid credentials provided")))
}

fn main() -> Result<()> {
    // Initialize color_eyre for beautiful error reporting
    color_eyre::install()?;
    
    println!("=== Testing Enhanced Error Reporting ===\n");
    
    // Test 1: Database Error
    println!("Test 1: Database Error");
    println!("=====================");
    if let Err(e) = simulate_database_error() {
        eprintln!("Error: {:?}", e);
        println!();
    }
    
    // Test 2: Authentication Error
    println!("Test 2: Authentication Error");
    println!("============================");
    if let Err(e) = simulate_auth_error() {
        eprintln!("Error: {:?}", e);
        println!();
    }
    
    // Test 3: Error Chain
    println!("Test 3: Error Chain");
    println!("===================");
    let result: Result<()> = Err(eyre!("Root cause: Network timeout")
        .wrap_err("Failed to connect to database")
        .wrap_err("User authentication failed"));
    
    if let Err(e) = result {
        eprintln!("Error Chain: {:?}", e);
        println!();
    }
    
    println!("=== Error Reporting Test Complete ===");
    Ok(())
}
