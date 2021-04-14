prog :=wap
release ?=
build :=armv7-unknown-linux-musleabihf
build_target :=--target $(build)
pi_location :=pi@192.168.0.170

ifdef release
  $(info release build...)
  release_flag :=--release
  release_location :=release
  extension :=release
else
  $(info debug build...)
  release_flag :=
  release_location :=debug
  extension :=debug
endif

build:
	cargo build $(build_target) $(release_flag)

install:
	scp target/$(build)/$(release_location)/$(prog) $(pi_location):~/$(prog)-$(extension)

all: build install

help:
	@echo "Usage: make $(prog) [release=1]"
