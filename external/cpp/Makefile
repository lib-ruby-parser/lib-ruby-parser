CXXFLAGS += -g -Wall -Wextra -std=c++17 -fPIC

OBJECTS = \
	structs.o \
	nodes.o \
	messages.o \
	bindings.o \
	bindings_nodes.o \
	bindings_messages.o

all: libstructures.a sizes
	./sizes > sizes-out

libstructures.a: $(OBJECTS)
	$(AR) rc libstructures.a $(OBJECTS)

%.o: %.cpp %.hpp
	$(CXX) -c -o $@ $< $(CXXFLAGS)

clean:
	rm -f *.o
	rm -f *.a
	rm -rf *.dSYM
	rm -f sizes

sizes: sizes.cpp
	$(CXX) sizes.cpp $(CXXFLAGS) -o sizes
