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
# Date : 27 Janvier 2023.
#
# Contexte : ce script a été écrit pour le projet Université de Rennes 1 / ISTIC / Master 2 - Cloud et Réseaux, millésime 2022-2023, intitulé "Rédiger un mode d'emploi d’utilisation du simulateur NS-3 pour simuler des réseaux mobiles Ad-Hoc - utilisant le protocole OLSR - réalistes".
#
# Commentaire : Ce projet implique donc de tester des configurations de noeuds en mode Ad-Hoc en réel, de les reproduire sur NS-3, puis d'ajuster les paramètres de NS-3 pour que le résultat de simulation s'approche au plus près de la réalité.
#
# Licence : free

 #  ============================
 # | FONCTION valider_selection |
 #  ============================

valider_selection () {
echo "Vous avez attribué à cette machine l'adresse IP $ip"
read -p "Validez vous cette sélection (O/N) ?" validate
case $validate in
	[Oo])
	echo "Vous avez attribué à cette machine l'adresse IP $ip"
	return 0
	;;
	[Nn])
	echo "Veuillez faire une autre sélection."
	return 1
	;;
	*)
	echo "Réponse non valide, veuillez réessayer."
	valider_selection
	;;
esac
}
#  =====================
# | FONCTION PRINCIPALE |
#  =====================
sudo systemctl daemon-reload
sudo systemctl stop olsrd2
ip1=10.1.0.1
ip2=10.4.0.4
ip3=10.7.0.7
ip4=10.16.0.16 
interface=`ip a | grep ": wlx00" | cut -c4-18`
echo "Le numéro de la carte Wifi installée est $interface"

# Utilisateur doit sélectionner une @IP parmis 4 choix
while true
do
    echo "Choisissez une machine :"
    # Correspond au PC-PF-ADHOC-1
    echo "1 pour une machine comme étant $ip1"
    # Correspond au PC-PF-ADHOC-4
    echo "2 pour une machine comme étant $ip2"
    # Correspond au PC-PF-ADHOC-7
    echo "3 pour une machine comme étant $ip3"
    # Correspond au PC-PF-ADHOC-16
    echo "4 pour une machine comme étant $ip4"
    echo "Q pour Quitter"

    read -p "Sélection : " selection

    case $selection in
        1) ip=$ip1
        valider_selection
		if [ $? -eq 0 ]; then
			break
		fi
	;;
        2) ip=$ip2
        valider_selection
		if [ $? -eq 0 ]; then
			break
		fi
	;;
        3) ip=$ip3
        valider_selection
		if [ $? -eq 0 ]; then
			break
		fi
	;;
        4) ip=$ip4
        valider_selection
		if [ $? -eq 0 ]; then
			break
		fi
	;;
        Q) 
        echo "Au revoir!"
        exit
	;;
        *) 
        echo "Sélection non valide, veuillez réessayer."
        ;;
    esac
done

sudo ifconfig $interface $ip netmask 255.255.255.224 #correspond au masque /27
sudo systemctl mask wpa_supplicant &&\
sudo systemctl stop wpa_supplicant &&\
sudo ip link set $interface down &&\
sudo iwconfig $interface mode ad-hoc &&\
sudo iwconfig $interface essid olsr &&\
sudo ip link set $interface up
sudo systemctl restart olsrd2

echo "Mode ad-hoc paramétré pour la carte Wifi $interface avec l'adresse $ip / 27. Cette adresse est seule dans son réseau."
