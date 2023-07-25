#!/bin/bash

cd /home/pi/mtvsetup;

sudo systemctl stop mtvsetup.service;

sudo rm -rf ./thumbnails;
mkdir ./thumbnails;

rm -rf ./mtv.db;
touch ./mtv.db;

git pull;

cargo build --release --bin mtvsetup;

rm /usr/local/bin/mtvsetup;
mv ./target/release/mtvsetup /usr/local/bin/mtvsetup;

chmod +xr /usr/local/bin/mtvsetup;
chown root:root /usr/local/bin/mtvsetup;

sudo systemctl start mtvsetup.service;

# /usr/local/bin/mtvsetup

# nohup ./target/release/mtvsetup >/dev/null 2>&1 &