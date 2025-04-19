all: compile test

compile:
	g++ -std=c++20 -O2 -Wall -pthread -pedantic -lyaml-cpp -o cybercraft cybercraft.cc
	cp cybercraft ../
test:
	mkdir -p  /tmp/cybercraft/; ./cybercraft --check; ls /tmp/cybercraft/; imv /tmp/cybercraft/checkProject.svg;

