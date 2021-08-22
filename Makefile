
build:
	cargo build

release:
	cargo build --release

install: release
	sudo install -s target/release/covid /usr/local/bin/

uninstall:
	sudo rm /usr/local/bin/covid

clean:
	cargo clean

.PHONY: build release install uninstall clean
