#!/bin/bash

# Build and run both services using Docker Compose
echo "🚀 Starting auth-service and app-service with Docker Compose..."

# Set JWT_SECRET environment variable
export JWT_SECRET=secret

# Build and start services
docker-compose up --build

echo "✅ Services should now be running at:"
echo "   App service: http://localhost:8000/"
echo "   Auth service: http://localhost:3000/"
