DEPS=src/main.rs Cargo.toml Makefile
TARGET=target/release/turn_off_display
DESTDIR=${PREFIX}/usr/local/

build: ${TARGET}

${TARGET}: ${DEPS}
	cargo build --release

clean: ${DEPS}
	cargo clean

install: ${TARGET}
	mkdir -p ${DESTDIR}/bin
	sudo install -o root -g root -m 4755 -t ${DESTDIR}/bin ${TARGET}
