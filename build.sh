#!/bin/bash
API_KEY="YOUR_API_KEY_HERE" \
FOLDER_ID="YOUR_FOLDER_ID_HERE" \
PATH=$PATH:$HOME/x-tools/arm-unknown-linux-gnueabi/bin/ \
CC=arm-unknown-linux-gnueabi-gcc \
cargo build --target=armv5te-unknown-linux-gnueabi --release -p sync-app
cp target/armv5te-unknown-linux-gnueabi/release/sync-app GoogleDriveSync.app
