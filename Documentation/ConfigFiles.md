# .cy_config.yaml
- In this configuration file you have the following options:
  ## Deactivate Encryption:
  ```yaml
  methode: none
  ```
  - This methode bypasses all encryption and is by default set.
  ---
  ## Yubikey Age Encryption:
  ```yaml
  methode: yubikey
  ```
  - This methode supports the yubikey encryption methode using the modified yubicrypt (frontend) libary.
  ---
  ## GPG Encryption: [To Be Implamented]
  ```yaml
  methode: gpg
  ```
  - This methode supports the gpg encryption methode using gpg and automated scripts for easy use.
  ```yaml
  keyname: randomGPG_Key
  ```
  - The name of key used for the gpg methode.
  ---
