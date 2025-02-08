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

.PHONY: web run build clean test
