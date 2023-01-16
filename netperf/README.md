## NETPERF

Cette partie du projet est un outil de benchmark que nous avons développé pour mesurer plusieurs caractéristiques réseau, à savoir:

- Le débit moyen
- Le Packet Drop Ratio (grâce au nombre de paquets transmis et au nombre de packets reçus)
- La latence moyenne
- X Les variations de latence
- La route utilisée par les paquets



Cet outil comporte 3 parties : 

- Un reader qui a pour but d'aller lire un fichier de config (nommé config.toml) et de le parser pour récupérer les addresses et ports nécessaires au benchmark.

- Un receiver ayant pour tầche d'ouvrir un socket TCP et de compter les paquets reçus et de pouvoir renvoyer périodiquement

- Un sender, qui s'occupe d'envoyer les paquets et de faire les mesures. l'exécution de ce dernier est divisée en 4 threads principaux :
  
  - Un thread **tcp_connection** qui va ouvrir une connection TCP jusqu'au receiver et la saturer pour déterminer le débit max. Il garde le compte du nombre de paquets envoyés.
    Il envoie également périodiquement des messages "update" pour que le receiver lui renvoie son compte actuel de paquets pour pouvoir calculer le drop ratio à un instant T.
  
  - Un thread **icmp_ping** qui s'occupe d'envoyer toutes les 200ms un ping à l'adresse cible et mesure le temps d'aller/retour pour déterminer la latence.
  
  - Un thread **icmp_route** qui détermine la route utilisée jusqu'à l'adresse cible en envoyant des requêtes d'écho ICMP avec un TTL croissant, et en examinant les réponses "TTL exceeded" pour retrouver les adresses des noeuds sur le chemin.
  
  - Et finalement un thread **sync** qui sert à synchroniser les affichages des différents threads, car nous avons implémenté, en plus de récupérer les données à la fin, un affichage périodique des valeurs, pour avoir un aperçu en temps réel des statistiques de la connection. Les threads gardent donc tous en mémoire des stats "partielles" qui séparent chaque print.



Cet outil est programmé dans le langage Rust car pour mesurer certaines caractéristiques il faut pouvoir utiliser des "raw sockets", il nous fallait donc un language bas niveau. Le Rust a des performances similaires au C mais est plus sûr dans sa gestion de la mémoire et fournit quelques abstractions assez utiles. Il était donc tout indiqué pour cette tầche. 
