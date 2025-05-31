.PHONY: all step0 step1 run0 run1

all: target/release/step0_repl target/release/step1_read_print

# Watch all Rust files in src/
SRC_FILES := $(shell find src -type f -name '*.rs')
# Optionally include Cargo files
CARGO_FILES := Cargo.toml Cargo.lock

# Build Targets
target/release/step0_repl: src/bin/step0_repl.rs $(SRC_FILES) $(CARGO_FILES)
	cargo build --release --bin step0_repl

target/release/step1_read_print: src/bin/step1_read_print.rs $(SRC_FILES) $(CARGO_FILES)
	cargo build --release --bin step1_read_print

step0: target/release/step0_repl
step1: target/release/step1_read_print

# Run Targets
run0: target/release/step0_repl
	./target/release/step0_repl

run1: target/release/step1_read_print
	./target/release/step1_read_print
