#!/bin/bash

echo "🔄 Syncing external examples from submodule..."

SUBMODULE_PATH=".submodules/stylus-sdk-rs/examples"
EXTERNAL_PATH="example_code/external"

# Check if submodule exists
if [ ! -d "$SUBMODULE_PATH" ]; then
    echo "❌ Submodule not found: $SUBMODULE_PATH"
    echo "Run: git submodule update --init --recursive"
    exit 1
fi

# Remove existing external directory
echo "🗑️  Cleaning existing external directory..."
rm -rf "$EXTERNAL_PATH"
mkdir -p "$EXTERNAL_PATH"

# Copy all examples from submodule
echo "📁 Copying examples from submodule..."
cp -r "$SUBMODULE_PATH"/* "$EXTERNAL_PATH/"

# Fix all Cargo.toml files
echo "🔧 Fixing Cargo.toml dependencies..."
find "$EXTERNAL_PATH" -name "Cargo.toml" -type f | while read -r file; do
    echo "Processing: $file"
    
    # Apply fixes
    sed -i '' \
        -e 's/stylus-sdk = { path = "[^"]*" }/stylus-sdk = "0.9.0"/g' \
        -e 's/stylus-sdk = { path = "[^"]*", features = \([^}]*\) }/stylus-sdk = { version = "0.9.0", features = \1 }/g' \
        -e 's/stylus-tools = { path = "[^"]*" }/stylus-tools = "0.2.0"/g' \
        -e 's/stylus-tools = { path = "[^"]*", features = \([^}]*\) }/stylus-tools = { version = "0.2.0", features = \1 }/g' \
        "$file"
    
    echo "✅ Fixed: $file"
done

echo "🎉 Sync complete! External examples are ready to use."