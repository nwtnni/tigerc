.PHONY: lib clean

lib: libtiger.o
	ar rcs libtiger.a libtiger.o

tiger.o : libtiger.c libtiger.h
	gcc -O3 -o libtiger.o -c libtiger.c

clean:
	rm libtiger.o libtiger.a
