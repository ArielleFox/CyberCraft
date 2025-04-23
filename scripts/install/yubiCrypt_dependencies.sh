#!/bin/bash

if [ -d ~/yubiCrypt ]; then
  echo "yubiCrypt Folder exists."
else
  cd ~/
  git clone https://github.com/ArielleFox/yubiCrypt.git
  cd yubiCrypt
  ./installer.py
fi
