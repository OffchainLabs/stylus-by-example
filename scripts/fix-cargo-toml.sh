#!/bin/bash

echo "Fixing Cargo.toml files in external examples..."
echo "Current directory: $(pwd)"

# Use -L to follow symbolic links
FILES=$(find -L example_code/external -name "Cargo.toml" -type f)

if [ -z "$FILES" ]; then
    echo "❌ No Cargo.toml files found!"
    exit 1
fi

echo "Found files:"
echo "$FILES"

# Process each file
echo "$FILES" | while read -r file; do
    if [ -n "$file" ]; then
        echo "Processing: $file"
        
        # Make backup
        cp "$file" "$file.backup"
        
        # Apply fixes for macOS
        sed -i '' \
            -e 's/stylus-sdk = { path = "[^"]*" }/stylus-sdk = "0.9.0"/g' \
            -e 's/stylus-sdk = { path = "[^"]*", features = \([^}]*\) }/stylus-sdk = { version = "0.9.0", features = \1 }/g' \
            -e 's/stylus-tools = { path = "[^"]*" }/stylus-tools = "0.2.0"/g' \
            -e 's/stylus-tools = { path = "[^"]*", features = \([^}]*\) }/stylus-tools = { version = "0.2.0", features = \1 }/g' \
            "$file"
        
        echo "✅ Processed: $file"
    fi
done

echo "✅ Script completed!"