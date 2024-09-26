#!/bin/bash
set -e

echo "Running database migrations..."
/usr/local/bin/migrate

echo "Starting the application..."
exec /usr/local/bin/rust-backend
