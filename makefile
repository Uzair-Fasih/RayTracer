run:
	@RUST_LOG=info cargo run > src/image.ppm

exec:
	@rustc ./src/main.rs
	@./main
