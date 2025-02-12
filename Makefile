web:
	python -m http.server 8654 -d _ &
	open http://localhost:8654

web\:stop:
	pkill -f http.server

run:
	cargo run

build:
	cargo build

clean:
	cargo clean

test:
	cargo test

server\:build:
	cd _ && go build -o server server.go

server\:run:
	cd _ && ./server

server:
	cd _ && go run server.go

.PHONY: web run build clean test server\:build server\:run server
