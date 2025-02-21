#!/bin/bash

# Check if commit message is provided
if [ $# -eq 0 ]; then
    echo "Error: Please provide a commit message"
    echo "Usage: $0 \"your commit message\""
    exit 1
fi

# Store the commit message
MESSAGE="$1"

# Add all changes
git add .

# Commit with provided message
git commit -m "$MESSAGE"

# Push to main branch
git push -u origin main --force
