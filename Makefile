.PHONY: deploy
deploy: ## Copy compiled files to docs directory
	rm -fr docs
	mkdir docs
	cargo web build --release
	cp target/wasm32-unknown-unknown/release/rustexp.js docs/
	cp target/wasm32-unknown-unknown/release/rustexp.wasm docs/
	cp static/* docs
