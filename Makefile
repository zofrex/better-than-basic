SRC_FILES = $(shell find src -name '*.rs')

target/debug/better-than-basic: $(SRC_FILES) Cargo.toml Cargo.lock
	cargo build
