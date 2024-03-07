run:
	@RUST_LOG=info cargo run > out/image.ppm

exec:
	@rustc ./src/main.rs
	@./main

clean:
	rm -rf out
	mkdir out