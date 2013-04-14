RUSTC := rustc
RUSTC_OPTS :=
SOURCE := sci.rc

all: build

build: $(SOURCE)
	$(RUSTC) $(RUSTC_OPTS) --lib sci.rc

test: clean
	$(RUSTC) $(RUSTC_OPTS) -L . --test sci.rc -o scitest~
	./scitest~

clean:
	rm -rf scitest~ *.dSYM *.dylib *.so:
