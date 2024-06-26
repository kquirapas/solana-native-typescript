#!/bin/bash

# Logging
TIMESTAMP=$(date +"%Y-%m-%d %T")
TRACE="[$TIMESTAMP] LOG:"
WARN="[$TIMESTAMP] WRN:"
ERROR="[$TIMESTAMP] ERR:"
SUCC="[$TIMESTAMP] SUCCESS:"

# Constants
HOST_FOLDER_ABSOLUTE_PATH=$(pwd)/target/verifiable-deploy
CONTAINER_FOLDER_ABSOLUTE_PATH=/app/target/deploy
IMAGE_NAME=program

# Build the image and name it "app"
# Prevent caching (start build from 0)
echo "$TRACE Building image..."
docker build -t $IMAGE_NAME --progress plain --no-cache .
echo "$SUCC Building image..."

# Run image and build the files
echo "$TRACE Building verifiable program..."
docker run --rm --name "build-$IMAGE_NAME" -v $HOST_FOLDER_ABSOLUTE_PATH:$CONTAINER_FOLDER_ABSOLUTE_PATH $IMAGE_NAME
echo "$SUCC Building verifiable program..."

# Delete the image named {$IMAGE_NAME}
echo "$TRACE Deleting app image..."
docker image rm $IMAGE_NAME
echo "$SUCC Deleting app image..."

