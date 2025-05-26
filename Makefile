target/release/step0_repl: src/bin/step0_repl.rs
	cargo build --release --bin step0_repl

target/release/step0_repl: src/bin/step1_read_print.rs
	cargo build --release --bin step1_read_print
