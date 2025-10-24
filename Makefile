build:
	cargo build
.PHONY: build

release:
	cargo build --release
.PHONY: release

clean:
	rm -rf target
.PHONY: clean
