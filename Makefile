RUSTC := rustc
RUSTC_OPTS := -L .
SOURCE := sci.rc
CC_OPTS := -fPIC

all: build

bindlib:
	gcc $(CC_OPTS) -c gslbind.c -o gslbind.o
	ar rcs libgslbind.a gslbind.o

build: $(SOURCE) bindlib
	$(RUSTC) $(RUSTC_OPTS) --lib sci.rc

test: clean bindlib
	$(RUSTC) $(RUSTC_OPTS) --test sci.rc -o scitest~
	./scitest~

clean:
	rm -rf scitest~ *.dSYM *.dylib *.so *.o *.a
