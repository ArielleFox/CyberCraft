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
        try:
            yield file
        except Exection as e:
            print(e)
        finally:
            print('Closing ')
            if file:
                file.close()

@contextmanager
def rm_file(path: str) -> Generator[IO, Any, None]:
    with timer():
        file: IO = open(path)
        print('Opening file')
        try:
            yield file
        except FileNotFoundError:
            print(f"Error: File '{file_name}' not found.")
        except Exection as e:
            print(e)
        finally:
            print('Closing file...')
            if file:
                file.close()
                os.remove(path)
                print(f"Removed '{path}' successfully.")


def decrypt_file(file_path):
    ident = "first.txt"
    curr = os.path.expanduser(f"~/.yubiCrypt/keys/{ident}")

    try:
        # Check if the identity file exists
        if not os.path.isfile(curr):
            raise FileNotFoundError(f"Identity file not found: {curr}")

        # Decrypt the file using `age`
        decrypted_file = file_path[::-1][4:][::-1]  # Remove ".age" from the file name
        command = ["age", "-d", "-i", curr, "-o", decrypted_file, file_path]
        subprocess.run(command, check=True)

        # Confirm successful decryption
        print(f"	SUCCESSFULLY DECRYPTED! {file_path} ==> {decrypted_file}")
        os.remove(file_path)  # Remove the encrypted file

    except Exception as e:
        # Handle decryption failure
        print("Reporting Failed Decryption Attempt")

        # Log failure details
        user = os.getlogin()
        hostname = socket.gethostname()
        timestamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        os_type = os.name

        failure_message = (
            f"401 Unauthorized {user}@{hostname}\n"
            f"{timestamp} {os_type}\n"
            "DECRYPTION FAILED"
        )
        print(failure_message)

        # Clean up keys directory
        keys_dir = os.path.expanduser("~/.yubiCrypt/keys/")
        for key_file in os.listdir(keys_dir):
            os.remove(os.path.join(keys_dir, key_file))

        # Log failure to a temporary file
        dirty_tmp_path = os.path.expanduser("~/.yubiCrypt/dirty.tmp")
        with file_manager(dirty_tmp_path, "a") as dirty_tmp:
            dirty_tmp.write(f"{failure_message}\n")
        print("Decryption failed.")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 decrypt.py <file_to_decrypt>")
        sys.exit(1)

    file_to_decrypt = sys.argv[1]
    if os.path.isfile(file_to_decrypt):
        print('Press the button on your yubikey: ')
        decrypt_file(file_to_decrypt)
    else:
        print(f"File not found: {file_to_decrypt}")
