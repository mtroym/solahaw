format:
	@cargo fix --allow-dirty
	@cargo fmt
	@cargo install cargo-machete
	@cargo machete --with-metadata --fix --skip-target-dir

lint:
	@cargo clippy

run:
	@make lint
	@make format
	@cargo run

clean:
	@cargo clean

linux:
	@cargo build --release
