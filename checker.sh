#!/bin/bash

# Fetch the latest version of stylus-sdk from crates.io
latest_version=$(curl -s https://crates.io/api/v1/crates/stylus-sdk | jq -r '.crate.max_version')

# Output the latest version
echo "The latest release of stylus-sdk on crates.io is $latest_version"

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
        sdk_version=$(grep 'stylus-sdk' Cargo.toml | awk -F'"' '{print $2}')
        
        if [ "$sdk_version" != "$latest_version" ]; then
          echo -e "\033[1;33mUpdating stylus-sdk version in $folder_name to $latest_version\033[0m"
          sed -i '' 's/stylus-sdk = ".*"/stylus-sdk = "'"$latest_version"'"/' Cargo.toml
          echo -e "\033[1;32mStylus-sdk version updated to $latest_version in $folder_name\033[0m"
        else
          echo -e "\033[1;32mStylus-sdk version is already $latest_version in $folder_name\033[0m"
        fi
      else
        echo -e "\033[1;31mStylus-sdk dependency not found in $folder_name\033[0m"
      fi

      # Run the cargo stylus check --no-verify command
      check_output=$(cargo stylus check --no-verify 2>&1)
      
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
echo -e "\n\033[1;32mAll checks and exports completed! Logs are available in 'check_results.log'.\033[0m"
