#!/bin/bash

echo "🚀 Starting SJS-PAMA Application Stack..."

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 1. Start Database & Redis
echo "📦 Starting Database and Redis via Docker Compose..."
if command_exists docker-compose; then
    docker-compose up -d
elif command_exists docker && docker compose version >/dev/null 2>&1; then
    docker compose up -d
else
    echo "⚠️  Warning: Neither 'docker-compose' nor 'docker compose' was found."
    echo "   Please ensure Docker is installed and running if you need the database."
fi

# Wait briefly for DB to be somewhat ready
sleep 2

# 2. Start Backend
echo "🦀 Starting Rust Backend..."
cd backend
cargo run &
BACKEND_PID=$!
cd ..

# 3. Start Frontend
echo "⚡ Starting SvelteKit Frontend..."
cd frontend
if command_exists bun; then
    bun install
    bun run dev -- --open &
elif command_exists npm; then
    npm install
    npm run dev -- --open &
else
    echo "❌ Error: Neither 'bun' nor 'npm' was found. Cannot start frontend."
fi
FRONTEND_PID=$!
cd ..

echo "======================================================="
echo "✅ Application startup sequence initiated!"
echo "Backend PID: $BACKEND_PID"
echo "Frontend PID: $FRONTEND_PID"
echo "Press [CTRL+C] at any time to stop everything."
echo "======================================================="

# Function to clean up background processes on script exit
cleanup() {
    echo ""
    echo "🛑 Stopping services..."
    [ -n "$FRONTEND_PID" ] && kill $FRONTEND_PID 2>/dev/null
    [ -n "$BACKEND_PID" ] && kill $BACKEND_PID 2>/dev/null
    echo "✅ Frontend and Backend stopped."
    echo "ℹ️  Docker containers (if started) are kept running in the background."
    exit 0
}

# Catch SIGINT (Ctrl+C) and SIGTERM to trigger cleanup
trap cleanup SIGINT SIGTERM

# Keep the script running to hold the trap active
wait

