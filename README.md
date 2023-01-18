# GPROJ

## 0-Materiel   
4 HP Probook 650 G1 (D9S33AV)     
4 clés Wifi AWUS036NEH (clé Wifi USB Alfa Network 320 mW et antenne 5 dBi), IEEE 802.11b/g/n USB 2.0Long-Range USB Adaptater   

NB: 
PC 1 - carte CC4 - 
PC 4 - carte 28B -
PC 7 - carte 272 -
PC 16 - carte CB9 -

## 1-Install Debian and OLSR on the four machines   

### 1.1-Install Debian
Debian ISO: firmware-11.5.0-amd64-DVD-1.iso   
Link: https://cdimage.debian.org/debian-cd/current/amd64/iso-dvd/   

### 1.2-Install OLSR   
OLSRd v2, commit ```fb15d54``` on Aug 25, 2022.   
Link: https://github.com/OLSR/OONF/tree/fb15d54d6a7a087cb0c5ec37c49804f6ce432396   
Use https://github.com/Carrybooo/GPROJ/blob/main/scripts/OLSR_Install.sh   
```bash
~/GPROJ/scripts$ ./OLSR_Install.sh
~/GPROJ/scripts$ ./SetAdHoc.sh
```
### 1.3-Start OLSR

Obtain $name_wifi_card
```ip a | grep ": wlx00" | cut -c4-18```

Modify olsrd2.conf   
Find it: ```find -name *.conf```   
Edit it: ```sudo nano olsrd2.conf``` 
Use the following configuration:
```
[interface=$name_wifi_card]
[interface=lo]
```
See at: http://www.olsr.org/mediawiki/index.php/OLSR_network_deployments   

Modifiy olsrd2.service
Find it: ```find -name *.service```   
Edit it: ```sudo nano olsrd2.service```
Change the following part in the service
```
[Service]
ExecStart=/usr/sbin/olsrd2_dynamic --load=/etc/olsrd2/olsrd2.conf
```

Ad-hoc network configuration
```
sudo systemctl mask wpa_supplicant
sudo systemctl stop wpa_supplicant
sudo ip link set $name_wifi_card down
sudo iwconfig $name_wifi_card mode ad-hoc
sudo iwconfig $name_wifi_card essid olsr
sudo ip link set $name_wifi_card up
sudo systemctl restart olsrd2
```
Start olsrd2
``` sudo systemctl start olsrd2.service```

Verify olsrd2 is running
``` sudo systemctl status olsrd2.service```

Stop olsrd2
``` sudo systemctl stop olsrd2.service```



