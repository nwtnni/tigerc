.PHONY: lib clean

lib: tiger.o
	ar rcs libtiger.a tiger.o

tiger.o : tiger.c tiger.h
	gcc -O3 -o tiger.o -c tiger.c

clean:
	rm tiger.o libtiger.a
