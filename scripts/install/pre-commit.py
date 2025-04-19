#!/usr/bin/env python3
"""Helper script to be used as a pre-commit hook."""
import os
import sys
import subprocess


def gitleaksEnabled():
    """Determine if the pre-commit hook for gitleaks is enabled."""
    out = subprocess.getoutput("git config --bool hooks.gitleaks")
    if out == "false":
        return False
    return True


if gitleaksEnabled():
    exitCode = os.WEXITSTATUS(os.system('gitleaks protect -v --staged'))
    if exitCode == 1:
        print('''Warning: gitleaks has detected sensitive information in your changes.
CyberCraft suggests:\
	never disable the giteaks precommit hook
	always make sure that it is enabled:          git config hooks.gitleaks true

Suggestion fixing secrets exposure:                   cybercraft --fix-secrets
''')
        sys.exit(1)
else:
    print('gitleaks precommit disabled\     (enable with `git config hooks.gitleaks true`)')
