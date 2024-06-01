build-binary:
	cargo build --release

gen-wasm-for-extension:
	rm -rf ./vscode_extension/core
	wasm-pack build --target bundler --out-dir ./vscode_extension/core --release

build-extension-base:
	make gen-wasm-for-extension
	yarn --cwd vscode_extension install
	yarn --cwd vscode_extension run package
	rm -rf package
	mkdir package

build-extension:
	make build-extension-base
	cd vscode_extension && vsce package --out ../package

publish-extension:
	make build-extension
	cd vscode_extension && ./publish.sh
	