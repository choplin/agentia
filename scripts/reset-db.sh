#!/bin/bash

# Get the app data directory based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    DB_PATH="$HOME/Library/Application Support/com.agentia.app/agentia.db"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    DB_PATH="$HOME/.local/share/com.agentia.app/agentia.db"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    # Windows
    DB_PATH="$APPDATA/com.agentia.app/agentia.db"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

if [ -f "$DB_PATH" ]; then
    echo "Removing database at: $DB_PATH"
    rm "$DB_PATH"
    echo "Database reset complete!"
else
    echo "No database found at: $DB_PATH"
fi

# Also remove the migrations table record if it exists
MIGRATIONS_PATH="${DB_PATH%/*}/migrations"
if [ -f "$MIGRATIONS_PATH" ]; then
    rm "$MIGRATIONS_PATH"
fi
