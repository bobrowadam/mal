release:
	cargo build --release
	cp -f ./target/release/step* .
	rm -r ./target

build:
	cargo build
	cp -f ./target/debug/step* .
	rm ./*.d
	rm -r ./target
