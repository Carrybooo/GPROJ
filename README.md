# GPROJ

## 0-Materiel   
4 HP Probook 650 G1 (D9S33AV)     
4 clés Wifi AWUS036NEH (clé Wifi USB Alfa Network 320 mW et antenne 5 dBi), IEEE 802.11b/g/n USB 2.0Long-Range USB Adaptater   

NB: les 4 machines doivent avoir des adresses privées sur des réseaux différents pour permettre de vérifier le fonctionnement ad-hoc     
PC 1 - carte wlx00c0ca959cc4 - IPv4 10.1.0.1/27   
PC 4 - carte wlx00c0caa7628b - IPv4 10.4.0.4/27
PC 7 - carte wlx00c0caa76272 - IPv4 10.7.0.7/27
PC 16 - carte wlx00c0ca959cb9 - IPv4 10.16.0.16/27

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
```
### 1.3-Verify parameters


**Modify olsrd2.conf** for each machine   
Find it: ```find -name *.conf```    
Edit it: ```sudo nano olsrd2.conf```   
Use the following configuration:  
```
[interface=$name_wifi_card]
  bindto $adress_IPv4
[interface=lo]
```
See at: http://www.olsr.org/mediawiki/index.php/OLSR_network_deployments   

**Modifiy olsrd2.service**   
Find it: ```find -name *.service```   
Edit it: ```sudo nano olsrd2.service```   
Change the following part in the service   
```
[Service]
ExecStart=/usr/sbin/olsrd2_dynamic --load=/etc/olsrd2/olsrd2.conf
```

**Verify ip_forward**
Vérifier que la valeur contenue dans ip_forward est 1. Si 0, la changer en 1 pour assurer le routage.      

```bash              
/proc/sys/net/ipv4: cat ip_forward         
```        

**Ad-hoc network configuration**    
```
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
```

```bash
~/GPROJ/scripts$ ./SetAdHoc.sh  
```

**Verify olsrd2 is running**    
``` sudo systemctl status olsrd2```

**Stop olsrd2**    
``` sudo systemctl stop olsrd2```



