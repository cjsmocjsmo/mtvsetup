#!/bin/bash
MTVP="/usr/share/mtvsetup"
MTVSTP=$MTVP"/mtvsetup"

if [ -f /etc/systemd/system/mtvsetup.service ]; then
    sudo systemctl stop mtvsetup.service;
fi

if [ ! -d $MTVP ]; then
    sudo mkdir $MTVP;
    sudo chmod +xwr $MTVP;
    cd $MTVP;
    git pull;
else
    cd $MTVP;
    git pull;
fi

if [ -d $MTVSTP/thumbnails ]; then
    sudo rm -rf $MTVSTP/thumbnails;
    mkdir $MTVSTP/thumbnails;
else
    mkdir $MTVSTP/thumbnails;
fi

if [ -f $MTVSTP/mtv.db ]; then
    sudo rm -rf $MTVSTP/mtv.db;
    touch $MTVSTP/mtv.db;
else
    touch $MTVSTP/mtv.db;
fi

cd $MTVSTP;

cargo build --release --bin mtvsetup;

sudo chmod +xr $MTVSTP/target/release/mtvsetup;
sudo chown root:root $MTVSTP/target/release/mtvsetup;
sudo mv $MTVSTP/target/release/mtvsetup /usr/bin/mtvsetup;

if [ ! -f /etc/systemd/system/mtvsetup.service ]; then
    sudo cp -pvr $MTVSTP/mtvsetup.service /etc/systemd/system/mtvsetup.service;
    sudo systemctl daemon-reload;
fi

sudo systemctl start mtvsetup.service;
sudo systemctl enable mtvsetup.service;














sudo systemctl start mtvsetup.service;
sudo systemctl status mtvsetup.service