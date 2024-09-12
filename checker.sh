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
    # Replace stylus-sdk with GitHub repo and branch, keeping features intact (GNU/BSD sed compatible)
    sed -i.bak -E 's#stylus-sdk = \{[[:space:]]*version = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'", features = [\1] }#' "$1" || sed -i '' -E 's#stylus-sdk = \{[[:space:]]*version = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'", features = [\1] }#' "$1"
    sed -i.bak -E 's#stylus-sdk = \{[^}]*version = "[^"]*".*\}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1" || sed -i '' -E 's#stylus-sdk = \{[^}]*version = "[^"]*".*\}#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1"
    # Handle plain version cases without features
    sed -i.bak -E 's#stylus-sdk = "[^"]*"#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1" || sed -i '' -E 's#stylus-sdk = "[^"]*"#stylus-sdk = { git = "'"$sdk_repo"'", branch = "'"$sdk_branch"'" }#' "$1"
  elif [ -n "$sdk_version" ]; then
    # Check for stylus-sdk with features and replace git with version, keeping features intact
    if grep -q 'features = \[' "$1"; then
      sed -i.bak -E 's#stylus-sdk = \{[[:space:]]*git = "[^"]*"[[:space:]]*,[[:space:]]*branch = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { version = "'"$sdk_version"'", features = [\1] }#' "$1" || sed -i '' -E 's#stylus-sdk = \{[[:space:]]*git = "[^"]*"[[:space:]]*,[[:space:]]*branch = "[^"]*"[[:space:]]*,[[:space:]]*features = \[([^]]*)\][[:space:]]*\}#stylus-sdk = { version = "'"$sdk_version"'", features = [\1] }#' "$1"
    else
      # Handle cases without features (switch to simple version string)
      sed -i.bak -E 's#stylus-sdk = \{[^}]*git = "[^"]*".*\}#stylus-sdk = "'"$sdk_version"'"#' "$1" || sed -i '' -E 's#stylus-sdk = \{[^}]*git = "[^"]*".*\}#stylus-sdk = "'"$sdk_version"'"#' "$1"
    fi
    # Handle plain git cases without features
    sed -i.bak -E 's#stylus-sdk = "[^"]*"#stylus-sdk = "'"$sdk_version"'"#' "$1" || sed -i '' -E 's#stylus-sdk = "[^"]*"#stylus-sdk = "'"$sdk_version"'"#' "$1"
  fi
}

# Function to process each directory
process_directory() {
  local base_dir=$1
  local dir_name=$(basename "$base_dir")

  echo -e "\n\033[1;34m============================================\033[0m"
  echo -e "\033[1;34mProcessing Directory: $dir_name\033[0m"
  echo -e "\033[1;34m============================================\033[0m"

  # Loop through each folder inside the base directory
  for dir in "$base_dir"/*/; do
    # Check if it's a directory
    if [ -d "$dir" ]; then
      folder_name=$(basename "$dir")
      echo -e "\n\033[1;36mEntering directory: $folder_name\033[0m"

      # Navigate into the directory
      cd "$dir" || continue

      # Check and update stylus-sdk version in Cargo.toml
      if grep -q 'stylus-sdk' Cargo.toml; then
        update_cargo_toml "Cargo.toml"
        echo -e "\033[1;32mUpdated Cargo.toml in $folder_name\033[0m"
      else
        echo -e "\033[1;31mStylus-sdk dependency not found in $folder_name\033[0m"
      fi

      # Run the cargo stylus check command
      check_output=$(cargo stylus check 2>&1)

      # Check if the command was successful
      if [ $? -eq 0 ]; then
        echo -e "\033[1;32mCheck passed in $folder_name\033[0m"
        check_status="PASSED"
      else
        echo -e "\033[1;31mCheck failed in $folder_name with error:\033[0m"
        echo -e "\033[1;31m$check_output\033[0m"
        check_status="FAILED"
      fi

      # Log the result of the check
      echo -e "\033[1;34m[$(date '+%Y-%m-%d %H:%M:%S')] $folder_name: Check $check_status\033[0m" >> ../../check_results.log

      # If the check passed, run the cargo stylus export-abi command
      if [ "$check_status" == "PASSED" ]; then
        echo -e "\033[1;33mRunning cargo stylus export-abi in $folder_name\033[0m"
        export_output=$(cargo stylus export-abi 2>&1)

        if [ $? -eq 0 ]; then
          echo -e "\033[1;32mExport ABI successful in $folder_name\033[0m"
        else
          echo -e "\033[1;31mExport ABI failed in $folder_name with error:\033[0m"
          echo -e "\033[1;31m$export_output\033[0m"
        fi
      fi

      # Go back to the base directory
      cd - > /dev/null
      echo -e "\033[1;36m---------------------------------\033[0m"
    fi
  done

  echo -e "\033[1;34mFinished processing $dir_name\033[0m"
  echo -e "\033[1;34m============================================\033[0m"
}

# Define the base directories
APPLICATIONS_DIR="example_code/applications"
BASIC_EXAMPLES_DIR="example_code/basic_examples"

# Process the applications directory
process_directory "$APPLICATIONS_DIR"

# Process the basic_examples directory
process_directory "$BASIC_EXAMPLES_DIR"

# Final message
echo -e "\n\033[1;32mAll checks and exports completed! Logs are available in '/tmp/check_results.log'.\033[0m"
