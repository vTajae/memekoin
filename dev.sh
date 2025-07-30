#!/bin/bash

# Exit on error
set -e

# Ensure we always kill Trunk on exit (even if wrangler fails)
cleanup() {
  echo "Stopping Trunk..."
  kill $TRUNK_PID
}
trap cleanup EXIT

# Start Trunk in watch mode, outputting to the axum-worker/static directory
cd leptos-wasm
trunk watch &
TRUNK_PID=$!

# Wait a moment to ensure Trunk has started (optional, but can help)
sleep 2

# Start the axum worker in watch mode (adjust as needed for your setup)
cd ../axum-worker
npx wrangler dev

# When wrangler exits, the cleanup trap will run
