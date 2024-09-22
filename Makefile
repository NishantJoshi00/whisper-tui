
.PHONY: run build

run: target/release/liz models/ggml-base.en.bin
	./target/debug/liz ./models/ggml-base.en.bin 2>/dev/null

build: target/release/liz models/ggml-base.en.bin
	@echo "Build complete"

target/release/liz: ./src/*
	cargo build --release

./models/ggml-base.en.bin:
	download-models.sh base.en

