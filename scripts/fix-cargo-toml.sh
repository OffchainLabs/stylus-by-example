#!/bin/bash

echo "Copying and fixing Cargo.toml files..."

# Copy files from submodule and fix them
find -L example_code/external -name "Cargo.toml" -type f | while read -r file; do
    echo "Processing: $file"
    
    # Get the real path (resolve symlink)
    real_file=$(readlink "$file")
    
    # Copy the file instead of modifying original
    cp "$real_file" "$file.tmp"
    
    # Apply fixes to the copy
    sed -i '' \
        -e 's/stylus-sdk = { path = "[^"]*" }/stylus-sdk = "0.9.0"/g' \
        -e 's/stylus-sdk = { path = "[^"]*", features = \([^}]*\) }/stylus-sdk = { version = "0.9.0", features = \1 }/g' \
        -e 's/stylus-tools = { path = "[^"]*" }/stylus-tools = "0.2.0"/g' \
        -e 's/stylus-tools = { path = "[^"]*", features = \([^}]*\) }/stylus-tools = { version = "0.2.0", features = \1 }/g' \
        "$file.tmp"
    
    # Replace the symlink with the fixed file
    rm "$file"
    mv "$file.tmp" "$file"
    
    echo "✅ Fixed: $file"
done

echo "✅ All files processed!"