all: build

build install clean test:
	@NixOS $@
