RUSTC ?= rustc
LIB=taglib

$(LIB)_files=$(wildcard src/*.rs)

$($LIB)_so): $($(LIB)_files
	mkdir -p lib
	$(RUSTC) src/lib.rs --out-dir=lib

doc: src/taglib.rs
	rustdoc src/taglib.rs

clean:
	rm -rf build/
	rm -rf doc/
