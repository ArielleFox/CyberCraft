from modules.yubiCryptLib import get_yubikey_serial as get_yubikey_serial
from modules.extract_identities import main as extract_ids
from modules.get_ids import main as get_ids
import os
import subprocess
import sys

SERIAL: str = get_yubikey_serial()


if __name__ == "__main__":
    get_ids()
    extract_ids()
    subprocess.run(['cp', 'formatted_identities.txt', '~/.yubiCrypt/keys/first.txt'], capture_output=False, text=True)
    print('GIL Status: ', sys._is_gil_enabled())  # Should return False

