# GPROJ

## 0-Materiel   
4 HP Probook 650 G1 (D9S33AV)     
4 clés Wifi AWUS036NEH (clé Wifi USB Alfa Network 320 mW et antenne 5 dBi), IEEE 802.11b/g/n USB 2.0Long-Range USB Adaptater   

NB: les 4 machines doivent avoir des adresses privées sur des réseaux différents pour permettre de vérifier le fonctionnement ad-hoc. Le script SetAdHocMan.sh permet de configurer n'importe quelle machine, possédant n'importe quelle carte Wifi, avec une des 4 adresses IPv4 suivantes.     
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
### 1.3-Configure and verify parameters

**Modify olsrd2.conf** for each machine   
Find it: ```find -name *.conf```    
Edit it: ```sudo nano olsrd2.conf```   
Use the following configuration:  
```
[interface=$name_wifi_card]
  bindto $adress_IPv4 # $adress_IPv4 à remplacer par 10.1.0.1/27 ou 10.4.0.4/27 ou 10.7.0.7/27 ou 10.16.0.16/27.
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
Configurer en mode ad-hoc, quasi-automatiquement (choix de 4 @IPv4), une machine possédant une carte Wifi branchée.
```bash
~/GPROJ/scripts$ ./SetAdHocMan.sh  
```

**Verify olsrd2 is running**    
``` sudo systemctl status olsrd2```

**Stop olsrd2**    
``` sudo systemctl stop olsrd2```

### 1.4-Control the OLSRv2 ad-hoc network    
Il existe plusieurs commandes pour vérifier l'état du réseau ad-hoc créé.      

Depuis un ordinateur autre que 10.1.0.1, **obtenir la route** pour arriver jusqu'à 10.0.1.0.1.     
```bash
sudo traceroute --udp 10.1.0.1
```
Depuis un ordinateur autre que 10.1.0.1, **visualiser en temps réel**, la route pour arriver jusqu'à 10.0.1.0.1.    
```bash
sudo watch traceroute --udp 10.1.0.1
 ```
Depuis un ordinateur autre que 10.1.0.1, **obtenir la route** pour arriver jusqu'à 10.0.1.0.1 **et des données de liaison imprimées dans un fichier csv (-4 = ipv4 / --udp == en udp / --csv == outpout en format csv /  -rwnb == besoin d'un rapport / -c 10 = envoie de dix paquets / -s 1448 = paquet de taille 1448 octets / -o pour les options / -i 1 = envoie d'un paquet toutes les 1s / adresse ip destinataire = 10.1.0.1 / awk 'etc' permet d'enregistrer des tabulations entre les résultats de la commande MTR pour faciliter l'exploitation des résultats en lignes et colonnes / Enregistrement des données dans fichier nommé Test_PC16to_PC01_20m.csv
```bash
mtr -4 --udp --csv -rwnb -c 10 -s 1448 -o "SR DL AW NBJXMI" -i 1 10.1.0.1 | awk '{print  $1"\t"$2"\t"$3"\t"$4"\t"$5"\t"$6"\t"$7"\t"$8"\t"$9"\t"$10"\t"$11"\t"$12"\t"$13"\t"$14"\t"$15}'> /home/debian/Résultats_distance/Test_PC16to_PC01_20m.csv
```



