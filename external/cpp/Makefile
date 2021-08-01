CXXFLAGS += -g -Wall -Wextra -std=c++17 -fPIC

OBJECTS = \
	byte.o \
	bytes.o \
	comment.o \
	diagnostic.o \
	error_level.o \
	list.o \
	loc.o \
	magic_comment.o \
	maybe_ptr.o \
	messages.o \
	node.o \
	ptr.o \
	shared_byte_list.o \
	source_line.o \
	string_ptr.o \
	token.o

all: libstructures.a

libstructures.a: $(OBJECTS)
	$(AR) rc libstructures.a $(OBJECTS)

%.o: %.cpp %.hpp
	$(CXX) -c -o $@ $< $(CXXFLAGS)

clean:
	rm -f *.o
	rm -f *.a
	rm -rf *.dSYM
	rm -f sizes

sizes: sizes.cpp $(OBJECTS)
	$(CXX) sizes.cpp $(CXXFLAGS) -o sizes

print-sizes: sizes
	./sizes
