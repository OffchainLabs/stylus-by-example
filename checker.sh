#!/bin/bash

# Define the base directory
BASE_DIR="example_code/applications"

# Loop through each folder inside the applications folder
for dir in "$BASE_DIR"/*/; do
  # Check if it's a directory
  if [ -d "$dir" ]; then
    echo "Entering directory: $dir"
    
    # Navigate into the directory
    cd "$dir" || continue
    
    # Run the cargo stylus check --no-verify command
    check_output=$(cargo stylus check --no-verify 2>&1)
    
    # Check if the command was successful
    if [ $? -eq 0 ]; then
      echo "Check passed in $(basename "$dir")"
      check_status="PASSED"
    else
      echo "Check failed in $(basename "$dir") with error:"
      echo "$check_output"
      check_status="FAILED"
    fi
    
    # Log the result of the check
    echo "$(basename "$dir"): Check $check_status" >> ../../check_results.log
    
    # If the check passed, run the cargo stylus export-abi command
    if [ "$check_status" == "PASSED" ]; then
      echo "Running cargo stylus export-abi in $(basename "$dir")"
      export_output=$(cargo stylus export-abi 2>&1)
      
      if [ $? -eq 0 ]; then
        echo "Export ABI successful in $(basename "$dir")"
      else
        echo "Export ABI failed in $(basename "$dir") with error:"
        echo "$export_output"
      fi
    fi
    
    # Go back to the base directory
    cd - > /dev/null
    
    echo "---------------------------------"
  fi
done
