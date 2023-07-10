#!/bin/bash

# Remove all files created by the build process.
sudo rm -rf //home/pi/mtvsetup/thumbnails;
mkdir /home/pi/mtvsetup/thumbnails;

rm -rf /home/pi/mtvsetup/mtv.db;
touch /home/pi/mtvsetup/mtv.db;

git pull;

cargo run --release --bin mtvsetup -- clean;