# CyberCraft
Devel notes
```c
// Python Version Suggestions: 3.14 (disabled GIL) For Best Results
```

## Installation
```bash
cd ~/
git clone https://github.com/ArielleFox/CyberCraft
cd CyberCraft
make
```

## First Run
```bash
./cybercraft init
./cybercraft --check
```
- edit .cy_config.yaml with the methode of use for example if you use yubikey switch the value from none to yubikey

## Version 0.5.61
Python version verified to work: 3.14

### 🧪 Example CLI Usages:
```bash
./cybercraft init         # initializes repository hardening
./cybercraft update       # self updates to the latest version
./cybercraft push                 # encrypts files in folder you are currently in ==> Git add ==> git push
./cybercraft pull                 # git pulls ==> decrypts files in folder you are currently in
./cybercraft --about              # tool description
./cybercraft --version            # version number
./cybercraft --check              # checks the current git repository + configurations
./cybercraft --encrypt            # folder-based via yubikey
./cybercraft --encrypt secrets.txt  # file-based
./cybercraft --decrypt            # folder-based via yubikey
./cybercraft --decrypt secrets.txt  # file-based
```

# Credits List (If any missing please submit an edited version via pull request <3*)
## Yubikey Manager
[repo](https://github.com/Yubico/yubikey-manager](https://github.com/Yubico/yubikey-manager)
[contributors](https://github.com/Yubico/yubikey-manager/graphs/contributors)

## Age
[repo](https://github.com/FiloSottile/age)
[contributors](https://github.com/FiloSottile/age/graphs/contributors)
 Authors:
 - Google LLC
 - Filippo Valsorda

## Age Plugin Yubikey
[repo](https://github.com/str4d/age-plugin-yubikey)
[contributors](https://github.com/str4d/age-plugin-yubikey/graphs/contributors)

## Pre Commit Hooks
[repo](https://github.com/pre-commit/pre-commit)
[fork](https://github.com/ArielleFox/pre-commit-hooks)

## Gitleaks
[repo](https://github.com/gitleaks/gitleaks)

# Attention
This release is functional but needs your help <3
Q: How can I help?
A: Inform me over any bugs, wishes or missing informations on credits via github. Thank you <3


* = Please don't sue me <3
