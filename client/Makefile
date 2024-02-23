install:
	cargo add yew --features csr
	cargo add serde_json
	cargo add serde --features derive
	cargo add uuid --features "serde v4 js"
	cargo add reqwasm
	cargo add gloo
	cargo add yewdux
	cargo add wasm-bindgen
	cargo add web-sys --features "HtmlInputElement Window"
	cargo add wasm-bindgen-futures

start-app:
	trunk serve --port 3000

dev:
	docker-compose -f ../docker-compose.dev.yml up -d client

dev-down:
	docker-compose -f ../docker-compose.dev.yml down

build:
	cargo build

run:
	cargo run