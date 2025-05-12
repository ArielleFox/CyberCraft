SRCS = cybercraft.cc Checking.cc Ploting.cc 
OBJS = $(SRCS:.cc=.o)
CXXFLAGS = -std=c++20 -O2 -Wall -pthread -pedantic
LIBS = -lyaml-cpp

all: compile anonymizer data_engine

cybercraft: $(OBJS)
	$(CXX) -o $@ $^ $(LIBS)

data_engine:
	mv ~/CyberCraft/src/data_engine/target/release/data_engine ./bin/
	cp bin/data_engine ~/.cybercraft/data_engine
	chmod +x ~/.cybercraft/data_engine

anonymizer:
	bash .compile.sh

clean:
	rm -f *.o cybercraft
	rm -f ~/CyberCraft/src/anonymizer/target/debug/anonymizer 
	rm -f ~/CyberCraft/src/data_engine/target/debug/data_engine
prepare:
	mv .objects/* .
	mv .headerfiles/* .

tidyup:
	mv Checking.o cybercraft.o Ploting.o ./.objects/
	mv Checking.hh Ploting.hh ./.headerfiles/


compile: clean prepare cybercraft tidyup
	mv cybercraft ./bin/
test:
	mkdir -p  /tmp/cybercraft/; ./cybercraft --check; ls /tmp/cybercraft/; imv /tmp/cybercraft/checkProject.svg;

