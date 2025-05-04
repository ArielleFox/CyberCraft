SRCS = cybercraft.cc Checking.cc Ploting.cc 
OBJS = $(SRCS:.cc=.o)
CXXFLAGS = -std=c++20 -O2 -Wall -pthread -pedantic
LIBS = -lyaml-cpp

all: compile

cybercraft: $(OBJS)
	$(CXX) -o $@ $^ $(LIBS)

clean:
	rm -f *.o cybercraft

compile: clean cybercraft
	cp cybercraft ../
test:
	mkdir -p  /tmp/cybercraft/; ./cybercraft --check; ls /tmp/cybercraft/; imv /tmp/cybercraft/checkProject.svg;

