#!/bin/bash

# A shell script to build and start Eden in the background, 
# and then compile Apple's TypeScript and run with Node.

echo -e "----- Appletree Blooming ----- "
# cd eden
echo -ne "[Appletree] Constructing \033[94mEden\033[m ..."
# cargo build --release
echo -e " \033[92mdone!\033[m"
echo -e "[Appletree] \033[94mEden\033[m launched."
# target/release/eden &
# (launch in the background)

# cd ..
echo -ne "[Appletree] Blooming \033[91mApple\033[m:"
npm start
echo -e " \033[92mdone!\033[m"
