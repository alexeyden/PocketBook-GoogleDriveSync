#!/bin/bash
PATH=$PATH:$HOME/x-tools/arm-unknown-linux-gnueabi/bin/ \
CC=arm-unknown-linux-gnueabi-gcc \
cargo build --target=armv5te-unknown-linux-gnueabi --release
cp target/armv5te-unknown-linux-gnueabi/release/sync-app GoogleDriveSync.app
