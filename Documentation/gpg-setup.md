# Step 1
edit .cy_config.yaml inside your repository
change methode: gpg
change keyname: <your@email.address>
# Step 2
Generate an new keypair
```bash
./cybercraft --newgpgkey
```
# Step 3
Test your configuration
``bash
./cybercraft --check
```
## Example output of an sucessfull .cy_config.yaml scan
```bash
❯ cybercraft --check
[Secure] .gitleaksignore not found.
methode ==> gpg
keyname ==> merica@fuck.yeah
[Found] .pre-commit-config.yaml
pre-commit installed at .git/hooks/pre-commit
[https://github.com/gitleaks/gitleaks] already up to date!
[https://github.com/ArielleFox/pre-commit-hooks] already up to date!

    ○
    │╲
    │ ○
    ○ ░
    ░    gitleaks

10:39AM INF 1 commits scanned.
10:39AM INF scanned ~1425 bytes (1.42 KB) in 14.1ms
10:39AM INF no leaks found
```
