#!/bin/bash

echo 'Checking Distrubution & Install Status'
./distro | grep 'Name: ' | cut -c 7-

NAME=$(grep 'Fedora' <<< $(./distro | grep 'Name: ' | cut -c 7-))
if [[ \$NAME == *Fedora* ]]; then
  if python3.11 -V; then echo "[Python3.11] [✓][Detected Distro Fedora]';" ; else sudo dnf install python3.11 libtomcrypt; fi;
fi

NAME=$(grep 'Fedora' <<< $(./distro | grep 'Name: ' | cut -c 7-))
if python3.14 -V; then echo "[Python3.14] [✓][Detected Distro Fedora]" ; else
  if [[ \$NAME == *Fedora* ]]; then
    sudo dnf install python3.14; sudo yum install libtomcrypt;
  fi;
fi

if python3.11 -V; then echo "[Python3.11] [✓][Detected Distro Ubuntu]" ; else
  if [[ \$NAME == *Ubuntu* ]]; then
    sudo apt-get install python3.11
  fi;
fi

if python3.14 -V; then echo "[Python3.14] [✓][Detected Distro Ubuntu]" ; else
  if [[ \$NAME == *Ubuntu* ]]; then
    sudo apt-get install python3.14
  fi;
fi



#echo 'Checking Distrubution & Install Status'
#./distro | grep 'Name: ' | cut -c 7-

#NAME=$(grep 'Fedora' <<< $(./distro | grep 'Name: ' | cut -c 7-))
#if [[ \$NAME == *Fedora* ]]; then if python3.11 -V; then echo "[Python3.11] [✓]" ; else /usr/bin/bash scripts/install/python3.11_dnf.sh; fi; then if python3.14 -V; then echo "[Python3.14] [✓]" ; else /usr/bin/bash scripts/install/python3.14_dnf.sh; fi; fi


#NAME=$(grep 'Ubuntu' <<< $(./distro | grep 'Name: ' | cut -c 7-))
#if python3.11 -V; then echo "[Python3.11] [✓]" ; else if [[ \$NAME == *Ubuntu* ]]; then  sudo apt-get install python3.11; fi; fi
#if python3.14 -V; then echo "[Python3.14] [✓]" ; else if [[ \$NAME == *Ubuntu* ]]; then sudo apt-get install python3.11; fi; fi
