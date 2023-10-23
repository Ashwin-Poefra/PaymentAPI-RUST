run:
	cargo lambda watch -a 127.0.0.1 -p 7171

build:
	cargo lambda build --release --arm64  --output-format zip