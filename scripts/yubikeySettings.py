#!/bin/env python3

from filemanager import file_manager
import time
import os

def append_file(filename: str, data: str, mode: str):
	with file_manager(filename, mode) as f:
		f.write(f'{data}\n')


def update_yubikey_id(filename: str):
	mode: str = 'a'
	with file_manager(filename,'r') as f:
		data = f.read()
		append_file('backupkey_formatted_identities.txt', data, 'w')
	with file_manager('backupkey_formatted_identities.txt', 'r') as f:
		data = f.read()
		os.system(f'cp ~/.cybercraft/.yubiCrypt/keys/first.txt {filename}')
		
def main():
	print('GIL Status: ', sys._is_gil_enabled())  # Should return False
	help_text = '''
command               description
---------------------|-----------------------------------------------
help                  Shows you all aviable commands
setup_backup_key      Guides you through backup yubikey intergration

'''
	cli = input(help_text)
	if cli == 'help':
		print(help_text)
	elif cli == 'setup_backup_key':
		print('Please pugin your backup yubikey')
		print('Continues in 3...')
		time.sleep(1)
		print('Continues in 2...')
		time.sleep(1)
		print('Continues in 1...')
		time.sleep(1)
		os.system('python3 ~/.yubiCrypt/yubiCryptImporter/import.py')
		update_yubikey_id('formatted_identities.txt')
		os.system('cd ~/.cybercraft; cat formatted_yubikey_identities.txt formatted_identities.txt > ~/.cybercraft/.yubiCrypt/keys/first.txt; cd -;')
		os.system('cp ~/.cybercraft/.yubiCrypt/keys/first.txt ~/.yubiCrypt/keys/first.txt')
main()
