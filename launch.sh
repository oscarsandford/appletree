#!/bin/bash
set -e

# A shell script to build and start Eden in the background, 
# and then compile Apple's TypeScript and run with Node.

echo -e " \033[92m----- Appletree Blooming -----\033[m "
cd eden
if [ ! -d "target/release" ] || [ ! -z "$1" ] && [ $1 == "build" ]; then
    echo -e "[\033[92mAppletree\033[m] Constructing \033[94mEden\033[m ..."
    cargo build --release
else
    echo -e "[\033[92mAppletree\033[m] Existing \033[94mEden\033[m build found!"
fi
echo -ne "[\033[92mAppletree\033[m] Launching \033[94mEden\033[m in the background ..."
target/release/eden & 
echo -e " \033[92mdone!\033[m"

cd ..
echo -e "[\033[92mAppletree\033[m] Launching \033[91mApple\033[m with npm start."
npm start
