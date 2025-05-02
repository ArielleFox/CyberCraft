#!/bin/bash

VERSION=$(cat .version.txt)
make
rm -rf ~/CyberCraft/yubiCrypt/modules/__pycache__
git add ~/CyberCraft/src/cybercraft
git commit -m 'Updated: $VERSION Binary Build'
git commit -a -m 'Updated: $VERSION'
git push 
