
# build web version and put it out directory
web_build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_forge.wasm
	cp -u bevy_forge/wasm/* out/
	cp -R -u bevy_forge/assets out/
	ls -R out

# self host web version, run web_build first
web_host:
	lwa_simple_server out

# validate the code
check:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

# installs cli tools used in the project
prepare:
	cargo install lwa_simple_server
	cargo install wasm-bindgen-cli