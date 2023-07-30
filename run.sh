#!/bin/bash
MTVP="/usr/share/mtvsetup/mtvsetup"
echo "stoping mtvsetup.service";
if [ -f /etc/systemd/system/mtvsetup.service ]; then
    sudo systemctl stop mtvsetup.service;
fi
cd $MTVP;
sudo rm -rf $MTVP/thumbnails;
sudo rm -rf $MTVP/mtv.db;
sudo rm -rf $MTVP/setup.mtv;
git pull;
cargo build --release --bin mtvsetup;
sudo mv ./target/release/mtvsetup /usr/bin/;
sudo chmod +xr /usr/bin/mtvsetup;
sudo chown root:root /usr/bin/mtvsetup;
# sudo systemctl daemon-reload;
# sudo systemctl start mtvsetup.service;
# sudo systemctl status mtvsetup.service