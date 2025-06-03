#!/bin/bash

# ----------------------------------------
# Parse --no-update flag (strip it from $@)
# ----------------------------------------
NO_UPDATE=false
ARGS=()
for arg in "$@"; do
  if [[ "$arg" == "--no-update" ]]; then
    NO_UPDATE=true
  else
    ARGS+=("$arg")
  fi
done
set -- "${ARGS[@]}"

# Default values for SDK version or repo
sdk_version=""
sdk_repo=""
sdk_branch=""
rpc_url="https://sepolia-rollup.arbitrum.io/rpc"

# Parse -v and -r flags
while getopts "v:r:" opt; do
  case ${opt} in
    v )
      sdk_version=$OPTARG
      ;;
    r )
      sdk_repo=$OPTARG
      sdk_branch=$(echo "$sdk_repo" | sed -n 's#.*/tree/\(.*\)#\1#p')
      sdk_repo=$(echo "$sdk_repo" | sed -n 's#\(.*\)/tree/.*#\1#p')
      if [ -z "$sdk_branch" ]; then
        sdk_repo=$OPTARG
        sdk_branch="main"
      fi
      ;;
    \? )
      echo "Usage: $0 [--no-update] [-v version] [-r repo_url]"
      exit 1
      ;;
  esac
done

# Fetch latest version if needed
get_latest_version() {
  curl -s https://crates.io/api/v1/crates/stylus-sdk | jq -r '.crate.max_version'
}

if [ -n "$sdk_repo" ]; then
  echo "Using SDK from GitHub repo: $sdk_repo (branch: $sdk_branch)"
elif [ -n "$sdk_version" ]; then
  echo "Using SDK version: $sdk_version"
else
  sdk_version=$(get_latest_version)
  echo "No flags provided. Using the latest SDK version: $sdk_version"
fi

# update_cargo_toml remains unchanged
update_cargo_toml() {
  local file="$1"
  
  if [ -n "$sdk_repo" ]; then
    # Update for git repo - handle features case
    sed -i -E 's#stylus-sdk = \{[[:space:]]*version = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'", features = [\1] }#' "$file"
    
    # Update for git repo - handle simple version case
    sed -i 's#stylus-sdk = "[^"]*"#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$file"
    
    # Update existing git entries
    sed -i 's#stylus-sdk = {[^}]*git = "[^"]*"[^}]*}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$file"
    
  elif [ -n "$sdk_version" ]; then
    if grep -q 'features = \[' "$file"; then
      # Handle git to version with features
      sed -i -E 's#stylus-sdk = \{[[:space:]]*git = "[^"]*"[[:space:]]*,[[:space:]]*branch = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { version = "'"$sdk_version"'", features = [\1] }#' "$file"
      
      # Handle version to version with features
      sed -i -E 's#stylus-sdk = \{[[:space:]]*version = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { version = "'"$sdk_version"'", features = [\1] }#' "$file"
    else
      # Handle simple cases - git to version
      sed -i 's#stylus-sdk = {[^}]*git = "[^"]*"[^}]*}#stylus-sdk = "'"$sdk_version"'"#' "$file"
      
      # Handle simple cases - version to version
      sed -i 's#stylus-sdk = "[^"]*"#stylus-sdk = "'"$sdk_version"'"#' "$file"
    fi
  fi
}

# Process each example folder
process_directory() {
  local base_dir=$1
  local dir_name
  dir_name=$(basename "$base_dir")

  echo -e "\n============================================"
  echo -e "Processing Directory: $dir_name"
  echo -e "============================================"

  for dir in "$base_dir"/*/; do
    if [ -d "$dir" ]; then
      folder_name=$(basename "$dir")
      echo -e "\nEntering directory: $folder_name"
      cd "$dir" || continue

      if grep -q 'stylus-sdk' Cargo.toml; then
        if [ "$NO_UPDATE" = false ]; then
          update_cargo_toml "Cargo.toml"
          echo -e "Updated Cargo.toml in $folder_name"
          # Clean up any backup files that might have been created
          find . -name "*.bak" -type f -delete
        else
          echo -e "Skipping SDK bump in $folder_name"
        fi
      else
        echo -e "Stylus-sdk dependency not found in $folder_name"
      fi

      # Run checks
      check_output=$(cargo stylus check -e $rpc_url 2>&1)
      if [ $? -eq 0 ]; then
        echo -e "Check passed in $folder_name"
        check_status="PASSED"
      else
        echo -e "Check failed in $folder_name with error:"
        echo -e "$check_output"
        check_status="FAILED"
      fi

      echo "[$(date '+%Y-%m-%d %H:%M:%S')] $folder_name: Check $check_status" \
        >> /tmp/check_results.log

      if [ "$check_status" == "PASSED" ]; then
        export_output=$(cargo stylus export-abi 2>&1)
        if [ $? -eq 0 ]; then
          echo -e "Export ABI successful in $folder_name"
        else
          echo -e "Export ABI failed in $folder_name with error:"
          echo -e "$export_output"
          echo "[$(date '+%Y-%m-%d %H:%M:%S')] $folder_name: Export ABI FAILED" \
            >> /tmp/check_results.log
          check_status="FAILED"
        fi
      fi

      cd - > /dev/null || exit
      echo -e "---------------------------------"
    fi
  done

  echo -e "Finished processing $dir_name"
  echo -e "============================================"
}

# Directories to process
APPLICATIONS_DIR="example_code/applications"
BASIC_EXAMPLES_DIR="example_code/basic_examples"

process_directory "$APPLICATIONS_DIR"
process_directory "$BASIC_EXAMPLES_DIR"

echo -e "\nAll checks and exports completed! Logs are available in '/tmp/check_results.log'."
