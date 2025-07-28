#!/bin/bash
set -e

# --- Config ---
SDK_REPO="https://github.com/OffchainLabs/stylus-sdk-rs.git"
EXAMPLES_PATH="example_code/external"
TMP_DIR="/tmp/stylus-sdk-rs-examples"

# --- Check for required tools ---
for cmd in jq curl git sed; do
  if ! command -v $cmd >/dev/null 2>&1; then
    echo "Error: $cmd is required but not installed." >&2
    exit 1
  fi
done

# --- Get latest stylus-sdk version (including betas) from crates.io ---
LATEST_VERSION=$(curl -s https://crates.io/api/v1/crates/stylus-sdk | jq -r '.crate.max_version')
echo "Latest stylus-sdk version: $LATEST_VERSION"

# --- Clean up any previous temp dir ---
rm -rf "$TMP_DIR"

git clone --depth 1 "$SDK_REPO" "$TMP_DIR"

# --- Copy examples ---
echo "Copying examples from SDK repo..."
rm -rf "$EXAMPLES_PATH"
mkdir -p "$EXAMPLES_PATH"
cp -r "$TMP_DIR/examples/"* "$EXAMPLES_PATH/"

# --- Update Cargo.toml files ---
# Iterate over each Cargo.toml file found in the examples directory
find "$EXAMPLES_PATH" -name "Cargo.toml" -type f -print0 | while IFS= read -r -d '' file; do
  echo "Updating $file"

  # Ensure the file is writable before attempting to modify it
  chmod +w "$file"

  # Update stylus-sdk dependencies
  # Pattern 1: { path = "...", features = [...] }
  sed -i '' -E "s~stylus-sdk = \{ *path = \"[^\"]*\" *, *features = ([^}]*)\}~stylus-sdk = { version = \"${LATEST_VERSION}\", features = \1 }~g" "$file"
  
  # Pattern 2: { path = "..." }
  sed -i '' -E "s~stylus-sdk = \{ *path = \"[^\"]*\" *\}~stylus-sdk = \"${LATEST_VERSION}\"~g" "$file"
  
  # Pattern 3: { version = "...", features = [...] }
  sed -i '' -E "s~stylus-sdk = \{ *version = \"[^\"]*\" *, *features = ([^}]*)\}~stylus-sdk = { version = \"${LATEST_VERSION}\", features = \1 }~g" "$file"
  
  # Pattern 4: "version"
  sed -i '' -E "s~stylus-sdk = \"[^\"]*\"~stylus-sdk = \"${LATEST_VERSION}\"~g" "$file"

  # Remove stylus-tools dependencies (not published yet)
  # Remove any stylus-tools dependency lines (various formats)
  sed -i '' -E "/^[[:space:]]*stylus-tools[[:space:]]*=/d" "$file"
  
  # Remove integration-tests feature that depends on stylus-tools
  sed -i '' -E "/^[[:space:]]*integration-tests[[:space:]]*=[[:space:]]*\[.*stylus-tools.*\]/d" "$file"
done

# --- Update Cargo.lock files ---
echo "Updating Cargo.lock files by running cargo build in each example..."
find "$EXAMPLES_PATH" -name "Cargo.toml" -type f -print0 | while IFS= read -r -d '' file; do
  dir=$(dirname "$file")
  echo "Running cargo build in $dir"
  
  # Change to the directory and run cargo build
  if (cd "$dir" && cargo build --quiet); then
    echo "✓ Successfully built $dir"
  else
    echo "⚠ Warning: cargo build failed in $dir"
    # Continue with other examples even if one fails
  fi
done

# --- Clean up temp files ---
rm -rf "$TMP_DIR"

echo "Done! All examples updated to stylus-sdk and stylus-tools v$LATEST_VERSION and Cargo.lock files refreshed."