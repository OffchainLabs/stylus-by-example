#!/bin/bash

# Default values for SDK version or repo
sdk_version=""
sdk_repo=""
sdk_branch=""

# Parse command-line arguments
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
        sdk_branch="main" # default to the main branch if not specified
      fi
      ;;
    \? )
      echo "Usage: $0 [-v version] [-r repo_url]"
      exit 1
      ;;
  esac
done

# Function to fetch the latest version of stylus-sdk from crates.io
get_latest_version() {
  curl -s https://crates.io/api/v1/crates/stylus-sdk | jq -r '.crate.max_version'
}

# Determine which SDK version or repo to use
if [ -n "$sdk_repo" ]; then
  echo "Using SDK from GitHub repo: $sdk_repo (branch: $sdk_branch)"
elif [ -n "$sdk_version" ]; then
  echo "Using SDK version: $sdk_version"
else
  sdk_version=$(get_latest_version)
  echo "No flags provided. Using the latest SDK version: $sdk_version"
fi

# Function to update Cargo.toml
update_cargo_toml() {
  if [ -n "$sdk_repo" ]; then
    # Replace stylus-sdk with GitHub repo and branch, keeping features intact if they exist
    sed -i.bak -E 's#stylus-sdk = \{[[:space:]]*version = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'", features = [\1] }#' "$1" || \
    sed -i '' 's#stylus-sdk = {[^}]*}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1"
    sed -i.bak 's#stylus-sdk = ".*"#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1" || \
    sed -i '' 's#stylus-sdk = ".*"#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1"
  elif [ -n "$sdk_version" ]; then
    # Check for stylus-sdk with features and replace git with version, keeping features intact
    if grep -q 'features = \[' "$1"; then
      sed -i.bak -E 's#stylus-sdk = \{[[:space:]]*git = "[^"]*"[[:space:]]*,[[:space:]]*branch = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { version = "'"$sdk_version"'", features = [\1] }#' "$1" || \
      sed -i '' 's#stylus-sdk = {[^}]*}#stylus-sdk = "'"$sdk_version"'"#' "$1"
    else
      # Handle cases without features (switch to simple version string)
      sed -i.bak 's#stylus-sdk = {[^}]*git = "[^"]*".*\}#stylus-sdk = "'"$sdk_version"'"#' "$1" || \
      sed -i '' 's#stylus-sdk = {[^}]*}#stylus-sdk = "'"$sdk_version"'"#' "$1"
    fi
  fi
}

# Function to process each directory
process_directory() {
  local base_dir=$1
  local dir_name=$(basename "$base_dir")

  echo -e "\n============================================"
  echo -e "Processing Directory: $dir_name"
  echo -e "============================================"

  # Loop through each folder inside the base directory
  for dir in "$base_dir"/*/; do
    if [ -d "$dir" ]; then
      folder_name=$(basename "$dir")
      echo -e "\nEntering directory: $folder_name"

      # Navigate into the directory
      cd "$dir" || continue

      # Check and update stylus-sdk version in Cargo.toml
      if grep -q 'stylus-sdk' Cargo.toml; then
        update_cargo_toml "Cargo.toml"
        echo -e "Updated Cargo.toml in $folder_name"
      else
        echo -e "Stylus-sdk dependency not found in $folder_name"
      fi

      # Run the cargo stylus check command
      check_output=$(cargo stylus check 2>&1)

      if [ $? -eq 0 ]; then
        echo -e "Check passed in $folder_name"
        check_status="PASSED"
      else
        echo -e "Check failed in $folder_name with error:"
        echo -e "$check_output"
        check_status="FAILED"
      fi

      # Log the result of the check to /tmp
      echo "[$(date '+%Y-%m-%d %H:%M:%S')] $folder_name: Check $check_status" >> /tmp/check_results.log

      # If check passed, run the cargo stylus export-abi command
      if [ "$check_status" == "PASSED" ]; then
        export_output=$(cargo stylus export-abi 2>&1)
        if [ $? -eq 0 ]; then
          echo -e "Export ABI successful in $folder_name"
        else
          echo -e "Export ABI failed in $folder_name with error:"
          echo -e "$export_output"
          # Log the export-abi failure to /tmp/check_results.log
          echo "[$(date '+%Y-%m-%d %H:%M:%S')] $folder_name: Export ABI FAILED" >> /tmp/check_results.log
          check_status="FAILED"
        fi
      fi

      # Go back to the base directory
      cd - > /dev/null
      echo -e "---------------------------------"
    fi
  done

  echo -e "Finished processing $dir_name"
  echo -e "============================================"
}

# Define the base directories
APPLICATIONS_DIR="example_code/applications"
BASIC_EXAMPLES_DIR="example_code/basic_examples"

# Process the directories
process_directory "$APPLICATIONS_DIR"
process_directory "$BASIC_EXAMPLES_DIR"

# Final message
echo -e "\nAll checks and exports completed! Logs are available in '/tmp/check_results.log'."
