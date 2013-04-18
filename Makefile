RUSTC := rustc
RUSTC_OPTS := -L .
SOURCE := sci.rc

all: build

bindlib:
	gcc -c gslbind.c -o gslbind.o
	ar rcs libgslbind.a gslbind.o

build: $(SOURCE) bindlib
	$(RUSTC) $(RUSTC_OPTS) --lib sci.rc

test: clean bindlib
	$(RUSTC) $(RUSTC_OPTS) --test sci.rc -o scitest~
	./scitest~

clean:
	rm -rf scitest~ *.dSYM *.dylib *.so *.o *.a
