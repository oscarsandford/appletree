#!/bin/bash

echo "Cleaning up work files..."
rm -rf dist
rm -rf node_modules
rm -rf eden/target
echo "Copying to temporary location to remove git repo..."
cp -r ../appletree /tmp
cd /tmp/appletree
rm -rf .git
echo "Done."