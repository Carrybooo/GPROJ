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

### 1.3-Start OLSR

Modify olsrd2.conf   
Find it: ```find -name *.conf```   
Edit it: ```sudo nano olsrd2.conf```   
See at: http://www.olsr.org/mediawiki/index.php/OLSR_network_deployments   
