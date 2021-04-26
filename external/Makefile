libstructures-c.a: structures.c
	$(CC) structures.c -g -c -Wall -o structures-c.o
	llvm-ar rc libstructures-c.a structures-c.o

libstructures-cpp.a: structures.cpp
	$(CXX) structures.cpp -g -std=c++17 -c -Wall -o structures-cpp.o
	llvm-ar rc libstructures-cpp.a structures-cpp.o

all: libstructures-c.a libstructures-cpp.a

clean:
	rm -f *.a
	rm -f *.o
