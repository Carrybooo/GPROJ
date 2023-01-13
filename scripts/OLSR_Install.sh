# Terminal à la racine
# sudo su
# cd /etc/apt
# nano sources.list
# mettre un #ligne 3
# sudo bash OLSR_Install.sh
sudo apt update -y && sudo apt upgrade -y &&\
sudo apt install -y git cmake build-essential libnl-3* gcc libtomcrypt-dev &&\
cd ~ &&\
sudo mkdir -p -m 777 OLSR &&\
sudo git clone https://github.com/OLSR/OONF OLSR &&\
cd OLSR/build &&\
sudo cmake .. &&\
sudo make &&\
echo Installation réussie