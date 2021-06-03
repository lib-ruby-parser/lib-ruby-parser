all: libstructures-c.a libstructures-cpp.a

C_HEADERS = \
	c/types/bytes.h \
	c/types/comment.h \
	c/types/diagnostic.h \
	c/types/magic_comment.h \
	c/types/node.h \
	c/types/source_line.h \
	c/types/token.h \
	c/declare_blob.h \
	c/declare_dummy_struct.h \
	c/list.h \
	c/maybe_ptr.h \
	c/ptr.h \
	c/shared_byte_list.h \
	c/string_ptr.h

CPP_HEADERS = \
	cpp/types/bytes.hpp \
	cpp/types/comment.hpp \
	cpp/types/diagnostic.hpp \
	cpp/types/magic_comment.hpp \
	cpp/types/node.hpp \
	cpp/types/source_line.hpp \
	cpp/types/token.hpp \
	cpp/declare_blob.hpp \
	cpp/declare_dummy_struct.hpp \
	cpp/list.hpp \
	cpp/maybe_ptr.hpp \
	cpp/ptr.hpp \
	cpp/shared_list.hpp \
	cpp/string_ptr.hpp

libstructures-c.a: structures.c $(C_HEADERS)
	$(CC) structures.c -g -c -Wall -Wextra -o structures-c.o
	$(AR) rc libstructures-c.a structures-c.o

libstructures-cpp.a: structures.cpp $(CPP_HEADERS)
	$(CXX) structures.cpp -g -std=c++17 -c -Wall -Wextra -fPIC -o structures-cpp.o
	$(AR) rc libstructures-cpp.a structures-cpp.o

clean:
	rm -f *.a
	rm -f *.o

sizes-c: structures.c $(C_HEADERS)
	$(CC) structures.c -g -DPRINT_SIZES -Wall -Wextra -o sizes-c

print-sizes-c: sizes-c
	./sizes-c

sizes-cpp: structures.cpp $(CPP_HEADERS)
	$(CXX) structures.cpp -g -std=c++17 -DPRINT_SIZES -Wall -Wextra -o sizes-cpp

print-sizes-cpp: sizes-cpp
	./sizes-cpp
