CXX=clang++
CC=clang

libstructures-c.a: structures.c
	$(CC) structures.c -g -c -Wall -Wextra -o structures-c.o
	llvm-ar rc libstructures-c.a structures-c.o

libstructures-cpp.a: structures.cpp
	$(CXX) structures.cpp -g -std=c++17 -c -Wall -Wextra -o structures-cpp.o
	llvm-ar rc libstructures-cpp.a structures-cpp.o

all: libstructures-c.a libstructures-cpp.a

clean:
	rm -f *.a
	rm -f *.o

sizes-c: structures.c
	$(CC) structures.c -g -DPRINT_SIZES -Wall -Wextra -o sizes-c

print-sizes-c: sizes-c
	./sizes-c

sizes-cpp: structures.cpp
	$(CXX) structures.cpp -g -std=c++17 -DPRINT_SIZES -Wall -Wextra -o sizes-cpp

print-sizes-cpp: sizes-cpp
	./sizes-cpp
