CFLAGS += -g -Wall -Wextra -fPIE

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

%.o: %.c %.h
	$(CC) -c -o $@ $< $(CFLAGS)

clean:
	rm -f *.o
	rm -f *.a
	rm -rf *.dSYM
	rm -f sizes

sizes: sizes.c $(OBJECTS)
	$(CC) sizes.c $(CFLAGS) -o sizes

check:
	CC=$(CC) ruby ../assert_defs.rb bindings.h bindings.c bindings_messages.c bindings_nodes.c
