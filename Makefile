TARGET ?= x86_64-unknown-linux-musl
TARGET_DIR = target/${TARGET}/release

all: ${TARGET_DIR}/sudokusolver-${TARGET}.tar.gz

${TARGET_DIR}/sudokusolver ${TARGET_DIR}/sudokusolver-benchmark:
	cargo build --release --target ${TARGET} --features html

${TARGET_DIR}/sudokusolver-${TARGET}.tar.gz: ${TARGET_DIR}/sudokusolver ${TARGET_DIR}/sudokusolver-benchmark
	cd ${TARGET_DIR} && tar -cvzf sudokusolver-${TARGET}.tar.gz sudokusolver sudokusolver-benchmark

clean:
	cargo clean

.PHONY: all clean
