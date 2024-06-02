#!/bin/bash
# MTVP="/usr/share/mtvsetup/mtvsetup"
# echo "stoping mtvsetup.service";
# if [ -f /etc/systemd/system/mtvsetup.service ]; then
#     sudo systemctl stop mtvsetup.service;
# fi
# cd $MTVP;
# sudo rm -rf $MTVP/thumbnails;
# sudo rm -rf $MTVP/db;
# mkdir $MTVP/db;
# touch $MTVP/db/mtv.db;
# sudo rm -rf $MTVP/setup.mtv;
# git pull;

rm /usr/share/mtvdb/mtv.db;
touch /usr/share/mtvdb/mtv.db;