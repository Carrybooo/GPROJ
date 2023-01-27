#!/bin/bash
# PC 1 - carte CC4 - 
# PC 4 - carte 28B -
# PC 7 - carte 272 -
# PC 16 - carte CB9 -

# Terminal à la racine
# sudo su
# cd /etc/apt
# nano sources.list
# mettre un #ligne 3

# En user et pas en root !
# chmod +x OLSR_Install.sh
# ./OLSR_Install.sh

sudo apt update -y && sudo apt upgrade -y &&\
sudo apt install -y git cmake build-essential libnl-3* gcc libtomcrypt-dev curl &&\
cd /home/debian/ &&\
if [ -d "/home/debian/OLSR" ]; then
	sudo rm -rfv /home/debian/OLSR
fi
sudo mkdir -p -m 777 OLSR &&\
sudo git clone https://github.com/OLSR/OONF OLSR &&\
cd OLSR/build &&\
sudo cmake .. &&\
sudo make &&\
sudo chown debian /home/debian/OLSR/apps/olsrd2/debian/olsrd2.conf &&\
sudo chown debian /home/debian/OLSR/apps/olsrd2/debian/olsrd2.service &&\

sudo mkdir -p /etc/olsrd2 &&\
sudo ln -sfv /home/debian/OLSR/apps/olsrd2/debian/olsrd2.conf /etc/olsrd2/ &&\
sudo ln -sfv /home/debian/OLSR/apps/olsrd2/debian/olsrd2.service /etc/systemd/system/ &&\
sudo mkdir -p /home/debian/OLSR/run &&\
sudo ln -sfv /home/debian/OLSR/apps/olsrd2/debian/olsrd2.conf /home/debian/OLSR/run/ &&\
sudo ln -sfv /home/debian/OLSR/apps/olsrd2/debian/olsrd2.service /home/debian/OLSR/run/ &&\
sudo ln -sfv /home/debian/OLSR/build/olsrd2_static /usr/sbin/ &&\
sudo ln -sfv /home/debian/OLSR/build/olsrd2_dynamic /usr/sbin/ &&\
sudo ln -sfv /home/debian/OLSR/build/olsrd2_dlep_static /usr/sbin/ &&\
sudo ln -sfv /home/debian/OLSR/build/olsrd2_dlep_dynamic /usr/sbin/ &&\
sudo systemctl daemon-reload &&\
# Install rustup pour compilation script RUST
sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo systemctl daemon-reload &&\
echo Installations OLSR et RUSTUP réussies
