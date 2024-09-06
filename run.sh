#!/bin/bash

echo "stoping mtvsetup.service";
if [ -f /etc/systemd/system/mtvsetup.service ]; then
    sudo systemctl stop mtvsetup.service;
fi

MTV="/usr/share/MTV";
MTVT="/usr/share/MTV/thumbnails"
MTVH=$HOME"/mtvsetup";

if [ ! -d $MTVP ]; then
    cd $MTVH;
    git clone https://github.com/cjsmocjsmo/mtvsetup.git
    # cd $MTVP;
    cargo run --release --bin mtvsetup;
    # sudo mv ./target/release/mtvsetup /usr/bin/;
    # sudo chmod +xr /usr/bin/mtvsetup;
    # sudo chown root:root /usr/bin/mtvsetup;
    # subprocess.run("/usr/bin/mtvsetup");
fi

if [ -d $MTVP ]; then
    sudo rm -rf $MTV/thumbnails;
    sudo rm -rf $MTV/setup.mtv;
    sudo rm -rf $MTV/db;
    sudo mkdir $MTV/thumbnails;
    sudo mkdir $MTV/db;
    cd $MTVH;
    git pull;
    cargo run --release --bin mtvsetup;
    # sudo mv ./target/release/mtvsetup /usr/bin/;
    # sudo chmod +xr /usr/bin/mtvsetup;
    # sudo chown root:root /usr/bin/mtvsetup;
    # subprocess.run("/usr/bin/mtvsetup");
fi

# if [ ! -f /etc/systemd/system/mtvsetup.service ]; then
#     sudo cp -pvr /usr/share/mtvsetup/mtvsetup/mtvsetup.service /etc/systemd/system/;
#     sudo chmod +xr /etc/systemd/system/mtvsetup.service;
#     sudo chown root:root /etc/systemd/system/mtvsetup.service;
#     sudo systemctl daemon-reload;
#     sudo systemctl enable mtvsetup.service;
#     sudo systemctl start mtvsetup.service;
# fi

# if [ -f /etc/systemd/system/mtvsetup.service ]; then
#     sudo systemctl start mtvsetup.service;
# fi



