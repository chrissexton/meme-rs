build:
	cargo build --bins --release

run: build
	./target/release/meme-rs

install: build
	cp ./target/release/meme-gen /usr/local/bin