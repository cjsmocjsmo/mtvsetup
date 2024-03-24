#!/bin/bash

echo "stoping mtvsetup.service";
if [ -f /etc/systemd/system/mtvsetup.service ]; then
    sudo systemctl stop mtvsetup.service;
fi

MTV="/usr/share/mtvsetup";
MTVP="/usr/share/mtvsetup/mtvsetup"

if [ ! -d $MTV ]; then
    sudo mkdir $MTV;
    sudo chmod +xr $MTV;
    sudo chown root:root $MTV;
fi

if [ ! -d $MTVP ]; then
    cd $MTV;
    git clone https://github.com/cjsmocjsmo/mtvsetup.git
    cd $MTVP;
    cargo build --release --bin mtvsetup;
    sudo mv ./target/release/mtvsetup /usr/bin/;
    sudo chmod +xr /usr/bin/mtvsetup;
    sudo chown root:root /usr/bin/mtvsetup;
fi

if [ -d $MTVP ]; then
    sudo rm -rf $MTVP/thumbnails;
    sudo rm -rf $MTVP/setup.mtv;
    sudo rm -rf $MTVP/db;
    mkdir $MTVP/db;
    cd $MTVP;
    git pull;
    cargo build --release --bin mtvsetup;
    sudo mv ./target/release/mtvsetup /usr/bin/;
    sudo chmod +xr /usr/bin/mtvsetup;
    sudo chown root:root /usr/bin/mtvsetup;
fi

if [ ! -f /etc/systemd/system/mtvsetup.service ]; then
    cp -pvr /usr/share/mtvsetup/mtvsetup/mtvsetup.service /etc/systemd/system/;
    sudo chmod +xr /etc/systemd/system/mtvsetup.service;
    sudo chown root:root /etc/systemd/system/mtvsetup.service;
    sudo systemctl daemon-reload;
    sudo systemctl enable mtvsetup.service;
    sudo systemctl start mtvsetup.service;
fi

if [ -f /etc/systemd/system/mtvsetup.service ]; then
    sudo systemctl start mtvsetup.service;
fi



