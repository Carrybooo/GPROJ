#!/bin/bash
#
# SetAdHocMan.sh
#
# Description : 
# Ce script a pour but de paramétrer facilement un ordinateur, muni d'une clé Wifi, pour fonctionner en mode Ad-Hoc avec 3 autres ordinateurs paramétrés avec le même script.
#
# Matériel utilisé:
# 1-ordinateur HP Probook 650 G1 (D9S33AV);
# 2-clé Wifi AWUS036NEH (clé Wifi USB Alfa Network 320 mW et antenne 5 dBi), IEEE 802.11b/g/n USB 2.0Long-Range USB Adaptater.
#
# Prérequis :
#  1-installer Debian 11.5 sur l'ordinateur (voir Readme à la racine);
#  2-installer OLSR v2 à l'aide du script OLSR_Install.sh (voir Readme à la racine).
#
# Auteurs : Walfroy BOUTIN ; Valentin GUERLESQUIN ; Ali JOUA.
# 
# Test : script bash fonctionnel.
#
# Date : 24 Janvier 2023.
#
# Contexte : ce script a été écrit pour le projet Université de Rennes 1 / ISTIC / Master 2 - Cloud et Réseaux, millésime 2022-2023, intitulé "Rédiger un mode d'emploi d’utilisation du simulateur NS-3 pour simuler des réseaux mobiles Ad-Hoc - utilisant le protocole OLSR - réalistes".
#
# Commentaire : Ce projet implique donc de tester des configurations de noeuds en mode Ad-Hoc en réel, de les reproduire sur NS-3, puis d'ajuster les paramètres de NS-3 pour que le résultat de simulation s'approche au plus près de la réalité.
#
# Licence : free

sudo systemctl daemon-reload
<<<<<<< Updated upstream
sudo systemctl stop olsrd2 
=======
sudo systemctl stop olsrd2
ip1=10.1.0.1
ip2=10.4.0.4
ip3=10.7.0.7
ip4=10.16.0.16 
>>>>>>> Stashed changes
interface=`ip a | grep ": wlx00" | cut -c4-18`
echo "Le numéro de la carte Wifi installée est $interface"
# Utilisateur doit sélectionner une @IP parmis 4 choix
while true
do
    echo "Choisissez une machine :"
    # Correspond au PC-PF-ADHOC-1
<<<<<<< Updated upstream
    echo "1 pour une machine avec @IP=10.1.0.1"
    # Correspond au PC-PF-ADHOC-4
    echo "4 pour une machine avec @IP=10.4.0.4"
    # Correspond au PC-PF-ADHOC-7
    echo "7 pour une machine avec @IP=10.7.0.7"
    # Correspond au PC-PF-ADHOC-16
    echo "16 pour une machine avec @IP=10.16.0.16"
=======
    echo "1 pour une machine comme étant $ip1"
    # Correspond au PC-PF-ADHOC-4
    echo "2 pour une machine comme étant $ip2"
    # Correspond au PC-PF-ADHOC-7
    echo "3 pour une machine comme étant $ip3"
    # Correspond au PC-PF-ADHOC-16
    echo "4 pour une machine comme étant $ip4"
>>>>>>> Stashed changes
    echo "Q pour Quitter"

    read -p "Sélection : " selection

    case $selection in
<<<<<<< Updated upstream
        1)  ip="10.1.0.1"
            break
            ;;
        4)  ip="10.4.0.4"
            break
            ;;
        7)  ip="10.7.0.7"
            break
            ;;
        16) ip="10.16.0.16"
=======
        1)  ip=$ip1
            break
            ;;
        2)  ip=$ip2
            break
            ;;
        3)  ip=$ip3
            break
            ;;
        4) ip=$ip4
>>>>>>> Stashed changes
            break
            ;;
        Q)  echo "Au revoir!"
            exit
            ;;
        *)  echo "Sélection non valide, veuillez réessayer."
            ;;
    esac
done
echo "Vous avez attribué à cette machine l'adresse IP $ip"
# Validation de l'adresse IP
while true; do
    read -p "Validez-vous cette sélection (O/N) ? " validate
    case $validate in
        [oO])
            echo "Vous avez validé la sélection de la machine $machine"
            break
            ;;
        [nN])
            echo "Veuillez faire une nouvelle sélection"
            break
            ;;
        *)
            echo "Réponse non valide, veuillez réessayer."
            ;;
    esac
done
<<<<<<< Updated upstream
# SELECT THE GOOD FOLLOWING LINE FOR EACH MACHINE AND COMMENT THE THREE OTHERS - HERE EXAMPLE FOR PC16
# sudo ifconfig $interface 10.1.0.1 netmask 255.255.255.224
# sudo ifconfig $interface 10.4.0.4 netmask 255.255.255.224
# sudo ifconfig $interface 10.7.0.7 netmask 255.255.255.224
sudo ifconfig $interface $ip netmask 255.255.255.224
=======

sudo ifconfig $interface $ip netmask 255.255.255.224 #masque /27
>>>>>>> Stashed changes
sudo systemctl mask wpa_supplicant &&\
sudo systemctl stop wpa_supplicant &&\
sudo ip link set $interface down &&\
sudo iwconfig $interface mode ad-hoc &&\
sudo iwconfig $interface essid olsr &&\
sudo ip link set $interface up
sudo systemctl restart olsrd2  
<<<<<<< Updated upstream
echo "Mode ad-hoc paramétré pour la carte $interface avec l'adresse $ip. Cette adresse est seule dans son réseau."
=======
echo "Mode ad-hoc paramétré pour la carte Wifi $interface avec l'adresse $ip / 27. Cette adresse est seule dans son réseau."
>>>>>>> Stashed changes
