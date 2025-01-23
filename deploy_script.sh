#!/bin/bash

# Function to display error message and exit
error_exit() {
    echo "$1" 1>&2
    exit 1
}

# get username and hostname from env variables  

set -a
source .env
set +a

# Check if username and hostname are set 
if [ -z "$DEPLOY_HOST" ] || [ -z "$DEPLOY_PATH" ]; then
    error_exit "Please set DEPLOY_HOST and PROJECT_PATH environment variables."
fi

tailwindcss -i styles/input.css -o styles/output.css  || error_exit "Failed to compile tailwindcss."

# Run trunk build --release
trunk build --release || error_exit "trunk build --release failed."

# Check if 'dist' folder exists
if [ ! -d "dist" ]; then
    error_exit "'dist' folder not found!"
fi

# SCP the folder to the server
rsync -avz  dist/* "$DEPLOY_HOST:$DEPLOY_PATH"

# Clean up
rm -r dist || error_exit "Failed to clean up dist folder."

# run remote script with ssh and env variables
ssh "$DEPLOY_HOST" "DEPLOY_PATH=$DEPLOY_PATH bash -s" < remote_deploy.sh 

echo "Deployment successful and folders cleaned up."


