#!/bin/bash

# Usage: ./download.sh <URL>
URL="$1"

if [ -z "$URL" ]; then
  echo "Usage: $0 <URL>"
  exit 1
fi

# Remove the protocol (http:// or https://) and domain, keep the path
URL_PATH=$(echo "$URL" | sed -E 's#https?://[^/]+##')

# Full local path inside public/
LOCAL_PATH="public$URL_PATH"

# Ensure the directory exists
mkdir -p "$(dirname "$LOCAL_PATH")"

# Download the file
curl -L "$URL" -o "$LOCAL_PATH"

echo "Downloaded $URL -> $LOCAL_PATH"
