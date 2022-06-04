all: test
	g++ -o test test.o -Ltarget/debug -lrust_cpp_playground

test: test.o
	g++ -c test.cc

test.o: test.cc target/debug/librust_cpp_playground.a rustlib.h

target/debug/librust_cpp_playground.a: src/lib.rs
	cargo build

rustlib.h: src/lib.rs
	cbindgen --config cbindgen.toml --crate rust_cpp_playground --output rustlib.h

clean:
	cargo clean
	rm -r *.o test rustlib.h
