#!/bin/bash
# ./SetAdHoc.sh
sudo systemctl daemon-reload
sudo systemctl stop olsrd2 
interface=`ip a | grep ": wlx00" | cut -c4-18`
echo "$interface"
# SELECT THE GOOD FOLLOWING LINE FOR EACH MACHINE AND COMMENT THE THREE OTHERS - HERE EXAMPLE FOR PC16
# sudo ifconfig $interface 10.1.0.1 netmask 255.255.255.224
# sudo ifconfig $interface 10.4.0.4 netmask 255.255.255.224
# sudo ifconfig $interface 10.7.0.7 netmask 255.255.255.224
sudo ifconfig $interface 10.16.0.16 netmask 255.255.255.224
sudo systemctl mask wpa_supplicant &&\
sudo systemctl stop wpa_supplicant &&\
sudo ip link set $interface down &&\
sudo iwconfig $interface mode ad-hoc &&\
sudo iwconfig $interface essid olsr &&\
sudo ip link set $interface up
sudo systemctl restart olsrd2  
ip=`ip a | grep "inet 10." | cut -c9-22`
echo "Mode ad-hoc paramétré pour la carte $interface avec l'adresse $ip"
