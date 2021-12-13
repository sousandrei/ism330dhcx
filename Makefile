host-test:
	$(eval TARGET = $(shell rustc -vV | sed -n 's|host: ||p'))
	cargo test --lib --tests --target=$(TARGET)
