#!/bin/bash
set -e

# A shell script to build and run both Apple and Eden 
# as background processes with nohup. Output is written
# to nohup.out in their respective directories.

echo -e " \033[92m----- Appletree Blooming -----\033[m "
# Compile Appletree
echo -e "\033[91mApple\033[m >> Installing dependencies..."
npm ci
echo -e "\033[91mApple\033[m >> Compiling..."
tsc
# Compile Eden
cd eden
echo -e "\033[94mEden\033[m >> Creating a release build..."
cargo build --release
# Launch Eden and then Apple 
# (the order is important, as Eden must be up in order to 
# process requests from Apple)
echo -e "\033[94mEden\033[m >> Startup."
nohup target/release/eden &
echo -e "\033[91mApple\033[m >> Startup."
nohup node dist/index.js &