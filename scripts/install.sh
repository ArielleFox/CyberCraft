#!/bin/bash


bash ./install/yubiCrypt_dependencies.sh

cd src; make all; cd -;

mkdir -p ~/.cybercraft/env/
mkdir -p ~/.cybercraft/.yubiCrypt/keys
mkdir -p ~/.cybercraft/.yubiCrypt/yubiCryptImporter/modules/

cp yubiCrypt/import.py ~/.cybercraft/.yubiCrypt/yubiCryptImporter/
cp -r yubiCrypt/modules/* ~/.cybercraft/.yubiCrypt/yubiCryptImporter/modules/
cp yubiCrypt/encrypt.py ~/.cybercraft/.yubiCrypt/encrypt.sh
cp yubiCrypt/decrypt.py ~/.cybercraft/.yubiCrypt/decrypt.sh

pip3.14 install pre-commit

gitleaks completion zsh >  ~/.zsh_autocompletion/gitleaks_completion.zsh
gitleaks completion bash > gitleaks_completion
sudo mv gitleaks_completion /etc/bash_completion.d/gitleaks_completion

cp scripts/install/pre-commit.py .git/hooks/pre-commit.py
cp .pre-commit-config.yaml ~/.cybercraft/.pre-commit-config.yaml

cp scripts/.gitignore ~/.cybercraft/.gitignore
cp scripts/gpggen ~/.cybercraft/gpggen; chmod +x ~/.cybercraft/gpggen;
bash scripts/set_user.sh

mv ./cybercraft ~/.local/bin/cybercraft
cp ~/CyberCraft/src/Folder-Anonymizer ~/.cybercraft/shell/Folder-Anonymizer
cp ~/CyberCraft/scripts/filemanager.py ~/.cybercraft/filemanager.py
cp ~/CyberCraft/scripts/yubikeySettings.py  ~/.cybercraft/yubikeySettings.py
