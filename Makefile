run:
	@cargo run

test:
	@cargo test -p pupactor_macro

test-all:
	@cargo test -p pupactor_macro
	@cargo test -p pupactor
	@cargo test -p pupactor_example
