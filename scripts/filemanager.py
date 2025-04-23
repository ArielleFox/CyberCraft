#!/bin/env python3.14
import os
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

def example():
    with file_manager('example.txt', 'w') as f:
        f.write('Hello World\n')

    with file_manager('example.txt', 'a') as f:
        f.write('May I offer you some brainrotten meme?\nNope')

    with file_manager('example.txt', 'r') as f:
        print(f.read())

    with rm_file('example.txt'):
        ...

if __name__ == '__main__':
    example()
