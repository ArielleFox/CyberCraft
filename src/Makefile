SRCS = cybercraft.cc Checking.cc Ploting.cc 
OBJS = $(SRCS:.cc=.o)
CXXFLAGS = -std=c++20 -O2 -Wall -pthread -pedantic
LIBS = -lyaml-cpp

all: clean compile rust_compile data_engine

cybercraft: $(OBJS)
	$(CXX) -o $@ $^ $(LIBS)

data_engine:
	mv ~/CyberCraft/src/data_engine/target/release/data_engine ./bin/
	cp bin/data_engine ~/.cybercraft/data_engine
	chmod +x ~/.cybercraft/data_engine

rust_compile:
	bash .compile.sh
	du -h ~/CyberCraft/src/bin/data_engine
	du -h ~/CyberCraft/src/bin/data_engine_smal

clean:
	rm -rf  data_engine/target

prepare:
	mv cc/* .
	mv objects/* .
	mv headerfiles/* .

tidyup:
	mv Checking.o cybercraft.o Ploting.o ./objects/
	mv Checking.hh Ploting.hh ./headerfiles/
	mv Checking.cc cybercraft.cc Ploting.cc ./cc/


compile: clean prepare cybercraft tidyup
	mv cybercraft ./bin/

test:
	mkdir -p  /tmp/cybercraft/; ./cybercraft --check; ls /tmp/cybercraft/; imv /tmp/cybercraft/checkProject.svg;

