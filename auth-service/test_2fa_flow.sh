#!/bin/bash

echo "🧪 Testing Complete 2FA Flow..."

# Set JWT secret
export JWT_SECRET=secret

# Start the service in background
echo "🚀 Starting auth service..."
cargo run > service.log 2>&1 &
SERVICE_PID=$!

# Wait for service to start
sleep 5

# Test user creation
echo "📝 Creating test user..."
curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{"email": "testflow@example.com", "password": "password123", "requires2FA": true}'

echo -e "\n🔐 Logging in to trigger 2FA..."
LOGIN_RESPONSE=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email": "testflow@example.com", "password": "password123"}')

echo "Login response: $LOGIN_RESPONSE"

# Extract login attempt ID
LOGIN_ATTEMPT_ID=$(echo $LOGIN_RESPONSE | grep -o '"loginAttemptId":"[^"]*"' | cut -d'"' -f4)
echo "Login attempt ID: $LOGIN_ATTEMPT_ID"

# Get the 2FA code from the service log
echo "📧 Looking for 2FA code in service logs..."
sleep 2
TWOFA_CODE=$(grep "Your verification code is:" service.log | tail -1 | grep -o '[0-9]\{6\}')
echo "2FA Code: $TWOFA_CODE"

if [ -n "$TWOFA_CODE" ]; then
    echo "✅ Found 2FA code: $TWOFA_CODE"
    
    echo "🔍 Verifying 2FA code..."
    VERIFY_RESPONSE=$(curl -s -X POST http://localhost:3000/verify-2fa \
      -H "Content-Type: application/json" \
      -d "{\"email\": \"testflow@example.com\", \"loginAttemptId\": \"$LOGIN_ATTEMPT_ID\", \"2FACode\": \"$TWOFA_CODE\"}")
    
    echo "Verify response: $VERIFY_RESPONSE"
    
    if [ -n "$VERIFY_RESPONSE" ]; then
        echo "✅ 2FA verification successful!"
    else
        echo "❌ 2FA verification failed"
    fi
else
    echo "❌ Could not find 2FA code in logs"
fi

# Cleanup
echo "🧹 Cleaning up..."
kill $SERVICE_PID
rm -f service.log

echo "🏁 Test complete!"
