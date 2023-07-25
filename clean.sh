#!/bin/bash

# Remove all files created by the build process.
sudo rm -rf //home/pi/mtvsetup/thumbnails;
mkdir /home/pi/mtvsetup/thumbnails;

rm -rf /home/pi/mtvsetup/mtv.db;
touch /home/pi/mtvsetup/mtv.db;

git pull;

cargo build --release --bin mtvsetup;

nohup /home/pi/mtvsetup/target/release/mtvsetup >/dev/null 2>&1 &