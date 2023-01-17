# ./SetAdHoc.sh
interface=`ip a | grep wlx00 | cut -c4-18`
echo "$interface"
sudo systemctl mask wpa_supplicant &&\
sudo systemctl stop wpa_supplicant &&\
sudo ip link set $interface down &&\
sudo iwconfig $interface mode ad-hoc &&\
sudo iwconfig $interface essid olsr &&\
sudo ip link set $interface up 
echo "Mode ad-hoc paramétré"
