default: build run

.PHONY: build
build:
	wasm-pack build

run: build
	cd www && npm start

start:
	firefox -new-tab "localhost:8080"
