#!/bin/bash

MTVSTP="/usr/share/mtvsetup/mtvsetup"

sudo systemctl stop mtvsetup.service;

git pull;

sudo rm -rf $MTVSTP/thumbnails;
mkdir $MTVSTP/thumbnails;

rm -rf $MTVSTP/mtv.db;
touch $MTVSTP/mtv.db;

cargo build --release --bin mtvsetup;

sudo chmod +xr $MTVSTP/target/release/mtvsetup;
sudo chown root:root $MTVSTP/target/release/mtvsetup;
sudo mv $MTVSTP/target/release/mtvsetup /usr/bin/mtvsetup;

sudo systemctl start mtvsetup.service;