#!/bin/env python3

import os
import subprocess
import datetime
import socket
import sys
import time
from contextlib import contextmanager
from typing import Generator, IO, Any
from datetime import datetime

@contextmanager
def timer() -> Generator[None, Any, None]:
    start_time: float = time.perf_counter()
    print(f'Started at: {datetime.now():%H:%M:%S}')
    try:
        yield
    finally:
        end_time: float = time.perf_counter()
        print(f'Ended at: {datetime.now():%H:%M:%S}')
        print(f'Time: {end_time - start_time:.4f}s')


@contextmanager
def file_manager(path: str, mode: str) -> Generator[IO, Any, None]:
    with timer():
        file: IO= open(path, mode)
        print('Opening file')
        try:
            yield file
        except Exection as e:
            print(e)
        finally:
            print('Closing file...')
            if file:
                file.close()




def encrypt_file(file_path):
    ident = "first.txt"
    curr = os.path.expanduser(f"~/.yubiCrypt/keys/{ident}")

    try:
        # Check if the identity file exists
        if not os.path.isfile(curr):
            raise FileNotFoundError(f"Identity file not found: {curr}")

        # Extract recipient key
        with file_manager(curr, "r") as ident_file:
            recipient_line = next(line for line in ident_file if "Recipient" in line)
            recipient_key = recipient_line[16:].strip()  # Extract key after "Recipient"

        # Encrypt the file using `age`
        encrypted_file = f"{file_path}.age"
        command = ["age", "-r", recipient_key, "-o", encrypted_file, file_path]
        subprocess.run(command, check=True)

        # Confirm successful encryption
        print(f"	SUCCESSFULLY ENCRYPTED! {file_path} ==> {encrypted_file}")
        os.remove(file_path)  # Remove the original file

    except Exception as e:
        # Handle encryption failure
        print("Reporting Failed Encryption Attempt")

        # Log failure details
        user = os.getlogin()
        hostname = socket.gethostname()
        timestamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        os_type = os.name

        failure_message = (
            f"401 Unauthorized {user}@{hostname}\n"
            f"{timestamp} {os_type}\n"
            "ENCRYPTION FAILED"
        )
        print(failure_message)

        # Log failure to a temporary file
        dirty_tmp_path = os.path.expanduser("~/.yubiCrypt/dirty.tmp")
        with file_manager(dirty_tmp_path, "a") as dirty_tmp:
            dirty_tmp.write(f"{failure_message}\n")

        print("Encryption failed.")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 encrypt.py <file_to_encrypt>")
        sys.exit(1)

    file_to_encrypt = sys.argv[1]
    if os.path.isfile(file_to_encrypt):
        encrypt_file(file_to_encrypt)
    else:
        print(f"File not found: {file_to_encrypt}")
