SRCS = cybercraft.cc Checking.cc Ploting.cc 
OBJS = $(SRCS:.cc=.o)
CXXFLAGS = -std=c++20 -O2 -Wall -pthread -pedantic
LIBS = -lyaml-cpp

all: compile anonymizer data_engine

cybercraft: $(OBJS)
	$(CXX) -o $@ $^ $(LIBS)

data_engine:
	cp ~/CyberCraft/src/data_engine/target/debug/data_engine ~/.cybercraft/data_engine
	chmod +x ~/.cybercraft/data_engine
	cp ~/CyberCraft/src/data_engine/target/debug/data_engine ./bin/

anonymizer:
	bash .compile.sh

clean:
	rm -f *.o cybercraft
	rm -f ~/CyberCraft/src/anonymizer/target/debug/anonymizer 
	rm -f ~/CyberCraft/src/data_engine/target/debug/data_engine
compile: clean cybercraft
	cp cybercraft ../
	cp cybercraft ./bin/
test:
	mkdir -p  /tmp/cybercraft/; ./cybercraft --check; ls /tmp/cybercraft/; imv /tmp/cybercraft/checkProject.svg;

