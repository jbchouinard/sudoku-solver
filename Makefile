all: dist/sudokusolver-linux-amd64.tar.gz

target/release/sudokusolver target/release/sudokusolver-benchmark:
	cargo build --release --features html

dist/sudokusolver: target/release/sudokusolver
dist/sudokusolver-benchmark: target/release/sudokusolver-benchmark
dist/sudokusolver dist/sudokusolver-benchmark:
	mkdir -p dist
	cp $< $@

dist/sudokusolver-linux-amd64.tar.gz: dist/sudokusolver dist/sudokusolver-benchmark
	cd dist && tar -cvzf sudokusolver-linux-amd64.tar.gz sudokusolver sudokusolver-benchmark

clean:
	cargo clean
	rm -rf dist

.PHONY: all clean
