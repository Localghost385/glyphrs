build-binary:
	cargo build --release

gen-wasm-for-extension:
	rm -rf ./vscode_extension/core
	wasm-pack build --target bundler --out-dir ./vscode_extension/core --release

build-extension:
	yarn --cwd vscode_extension install
	yarn --cwd vscode_extension run package
	mkdir package
	cd vscode_extension && vsce package --out ../package
