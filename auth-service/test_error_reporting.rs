use auth_service::{
    domain::{Email, Password, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

fn main() {
    // Initialize color_eyre for beautiful error reporting
    color_eyre::install().expect("Failed to install color_eyre");
    
    // Initialize tracing
    auth_service::utils::tracing::init_tracing().expect("Failed to initialize tracing");

    println!("=== Testing Enhanced Error Reporting ===\n");

    // Test 1: Invalid email parsing
    println!("1. Testing invalid email parsing:");
    match Email::parse("invalid-email".to_string()) {
        Ok(_) => println!("   ✅ Unexpected success"),
        Err(e) => {
            println!("   ❌ Expected error:");
            println!("   Error: {:?}", e);
        }
    }

    // Test 2: Invalid password parsing
    println!("\n2. Testing invalid password parsing:");
    match Password::parse("123".to_string()) {
        Ok(_) => println!("   ✅ Unexpected success"),
        Err(e) => {
            println!("   ❌ Expected error:");
            println!("   Error: {:?}", e);
        }
    }

    // Test 3: Invalid login attempt ID parsing
    println!("\n3. Testing invalid login attempt ID parsing:");
    match LoginAttemptId::parse("not-a-uuid".to_string()) {
        Ok(_) => println!("   ✅ Unexpected success"),
        Err(e) => {
            println!("   ❌ Expected error:");
            println!("   Error: {:?}", e);
        }
    }

    // Test 4: Invalid 2FA code parsing
    println!("\n4. Testing invalid 2FA code parsing:");
    match TwoFACode::parse("123".to_string()) {
        Ok(_) => println!("   ✅ Unexpected success"),
        Err(e) => {
            println!("   ❌ Expected error:");
            println!("   Error: {:?}", e);
        }
    }

    // Test 5: Valid email and auth cookie generation
    println!("\n5. Testing valid email and auth cookie generation:");
    match Email::parse("test@example.com".to_string()) {
        Ok(email) => {
            match generate_auth_cookie(&email) {
                Ok(cookie) => {
                    println!("   ✅ Successfully generated auth cookie");
                    println!("   Cookie name: {}", cookie.name());
                    println!("   Cookie value length: {}", cookie.value().len());
                }
                Err(e) => {
                    println!("   ❌ Failed to generate auth cookie:");
                    println!("   Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to parse email:");
            println!("   Error: {:?}", e);
        }
    }

    println!("\n=== Error Reporting Test Complete ===");
}
