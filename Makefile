PREFIX     ?= /usr
BINDIR      = $(DESTDIR)$(PREFIX)/bin
MANDIR      = $(DESTDIR)$(PREFIX)/share/man/man1
LICDIR      = $(DESTDIR)$(PREFIX)/share/licenses/stardial
CARGOFLAGS ?=

BINARY      = target/release/stardial

.PHONY: all build check install uninstall clean

all: build

build:
	cargo build --release $(CARGOFLAGS)

check:
	cargo test $(CARGOFLAGS)

install:
	install -Dm755 $(BINARY)          $(BINDIR)/stardial
	install -Dm644 man/stardial.1     $(MANDIR)/stardial.1
	install -Dm644 LICENSE            $(LICDIR)/LICENSE

uninstall:
	rm -f  $(BINDIR)/stardial
	rm -f  $(MANDIR)/stardial.1
	rm -rf $(LICDIR)

clean:
	cargo clean
