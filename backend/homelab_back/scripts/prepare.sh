#!/bin/bash

set -e

echo "🛠️  Preparing SQLx for NAS Server..."
cargo sqlx prepare --package nas-server

echo "🛠️  Preparing SQLx for User Server..."
cargo sqlx prepare --package user-server

echo "✅ All done! You are ready to push."