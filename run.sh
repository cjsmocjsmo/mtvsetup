#!/bin/bash
MTVP="/usr/share/mtvsetup"
MTVSTP=$MTVP"/mtvsetup"

echo "stoping mtvsetup.service";
if [ -f /etc/systemd/system/mtvsetup.service ]; then
    sudo systemctl stop mtvsetup.service;
fi

echo "doing git pull";
if [ ! -d $MTVP ]; then
    sudo mkdir $MTVP;
    sudo chmod +xwr $MTVP;
    cd $MTVP;
    git clone https://github.com/cjsmocjsmo/mtvsetup.git;
fi

if [ -d $MTVSTP ]; then
    cd $MTVSTP;
    git pull;
fi

echo "removing thumbnails";
if [ -d $MTVSTP/thumbnails ]; then
    sudo rm -rf $MTVSTP/thumbnails;
    mkdir $MTVSTP/thumbnails;
else
    mkdir $MTVSTP/thumbnails;
fi

echo "removing mtv.db";
if [ -f $MTVSTP/mtv.db ]; then
    sudo rm -rf $MTVSTP/mtv.db;
    touch $MTVSTP/mtv.db;
else
    touch $MTVSTP/mtv.db;
fi

echo "building mtvsetup";
cd $MTVSTP;
cargo build --release --bin mtvsetup;

echo "moving mtvsetup";
sudo chmod +xr $MTVSTP/target/release/mtvsetup;
sudo chown root:root $MTVSTP/target/release/mtvsetup;
sudo mv $MTVSTP/target/release/mtvsetup /usr/bin/mtvsetup;

echo "copying mtvsetup.service if not exists";
if [ ! -f /etc/systemd/system/mtvsetup.service ]; then
    sudo cp -pvr $MTVSTP/mtvsetup.service /etc/systemd/system/mtvsetup.service;
    sudo chown root:root /etc/systemd/system/mtvsetup.service;
    sudo chmod +wr /etc/systemd/system/mtvsetup.service;
    sudo systemctl daemon-reload;
fi

echo "starting mtvsetup.service";
sudo systemctl start mtvsetup.service;

echo "enabling mtvsetup.service";
sudo systemctl enable mtvsetup.service;














sudo systemctl start mtvsetup.service;
sudo systemctl status mtvsetup.service