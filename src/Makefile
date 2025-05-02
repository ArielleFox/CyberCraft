all: compile

compile:
	g++ -std=c++20 -O2 -Wall -pthread -pedantic -lyaml-cpp -o cybercraft cybercraft.cc Checking.cc Ploting.cc
	cp cybercraft ../
test:
	mkdir -p  /tmp/cybercraft/; ./cybercraft --check; ls /tmp/cybercraft/; imv /tmp/cybercraft/checkProject.svg;

