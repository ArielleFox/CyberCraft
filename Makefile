all: clean build install

installvenv: reinstallvenv
	python3 -m venv ~/.cybercraft/cybercraft-venv; source ~/.cybercraft/cybercraft-venv/bin/activate; python3 -m pip install -r ~/.cybercraft/requirements.txt; deactivate;

reinstallvenv:
	rm -rf ~/.cybercraft/cybercraft-venv

audit: installvenv
	pip-audit --fix --requirement requirements.txt

test: build
	rm -rf /tmp/valgrind_cybercraft/
	mkdir -p /tmp/valgrind_cybercraft/compilation-tests/
	valgrind -s ./cybercraft > /tmp/valgrind_cybercraft/compilation-tests/cybercraftbin.log
	valgrind -s ./cybercraft init >> /tmp/valgrind_cybercraft/compilation-tests/cybercraftbin.log
	valgrind -s ./cybercraft --check >> /tmp/valgrind_cybercraft/compilation-tests/cybercraftbin.log
	valgrind -s ./cybercraft --about >> /tmp/valgrind_cybercraft/compilation-tests/cybercraftbin.log
	cat /tmp/valgrind_cybercraft/compilation-tests/cybercraftbin.log
	imv /tmp/cybercraft/checkProject.svg

install:
	# Creating Directories
	mkdir -p ~/.cybercraft/art/
	mkdir -p ~/.cybercraft/shell/
	# Copying Data
	cp src/art/mascot ~/.cybercraft/art/
	cd scripts; bash install.sh; cd -;
	
build:
	cd src;	make compile; cd ../

buildclean: build

clean:
	rm ~/.local/bin/cybercraft
	touch ~/.local/bin/cybercraft
