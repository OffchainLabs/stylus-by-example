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
echo "🔧 Fixing Cargo.toml dependencies to use submodule paths..."
find "$EXTERNAL_PATH" -name "Cargo.toml" -type f | while read -r file; do
    echo "Processing: $file"
    
    # Fix path dependencies to point to our submodule
    # From: { path = "../../stylus-sdk" }
    # To:   { path = "../../../.submodules/stylus-sdk-rs/stylus-sdk" }
    sed -i '' \
        -e 's|stylus-sdk = { path = "[^"]*" }|stylus-sdk = { path = "../../../.submodules/stylus-sdk-rs/stylus-sdk" }|g' \
        -e 's|stylus-sdk = { path = "[^"]*", features = \([^}]*\) }|stylus-sdk = { path = "../../../.submodules/stylus-sdk-rs/stylus-sdk", features = \1 }|g' \
        -e 's|stylus-tools = { path = "[^"]*" }|stylus-tools = { path = "../../../.submodules/stylus-sdk-rs/stylus-tools" }|g' \
        -e 's|stylus-tools = { path = "[^"]*", features = \([^}]*\) }|stylus-tools = { path = "../../../.submodules/stylus-sdk-rs/stylus-tools", features = \1 }|g' \
        "$file"
    
    echo "✅ Fixed: $file"
done

# Build all examples to generate Cargo.lock files
echo ""
echo "🔨 Building all examples to generate Cargo.lock files..."
echo "=================================================="

BUILD_SUCCESS=0
BUILD_FAILURE=0

for dir in "$EXTERNAL_PATH"/*/; do
    if [ -d "$dir" ] && [ -f "$dir/Cargo.toml" ]; then
        folder_name=$(basename "$dir")
        echo ""
        echo "🔨 Building: $folder_name"
        
        cd "$dir" || continue
        
        # Run cargo build
        if cargo build > /dev/null 2>&1; then
            echo "✅ Build successful: $folder_name"
            BUILD_SUCCESS=$((BUILD_SUCCESS + 1))
        else
            echo "❌ Build failed: $folder_name"
            echo "   Running with output for debugging..."
            cargo build 2>&1 | head -10 | sed 's/^/   /'
            BUILD_FAILURE=$((BUILD_FAILURE + 1))
        fi
        
        cd - > /dev/null || exit
    fi
done

echo ""
echo "📊 Build Summary:"
echo "  ✅ Successful: $BUILD_SUCCESS"
echo "  ❌ Failed: $BUILD_FAILURE"

if [ $BUILD_FAILURE -gt 0 ]; then
    echo ""
    echo "⚠️  Some builds failed. You may need to manually fix these examples."
    echo "   However, most examples should now work with 'cargo stylus check'."
fi

echo ""
echo "🎉 Sync complete! External examples are ready to use with submodule path dependencies"
echo "📝 Note: Using path dependencies to submodule ensures perfect compatibility"
echo "🚀 You can now run 'cargo stylus check' in any external example directory"