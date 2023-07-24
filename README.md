circle-rust
===========

This project is a very first attempt to support applications, written in the Rust programming language, in accessing Circle bare-metal libraries.

Prerequisites
-------------

The following tools must be installed on your development host:

* GNU-C/C++-based toolchain(s), which allow to build Circle for *arm-none-eabi* (32-bit) and/or *aarch64-none-elf* (64-bit) targets. See the Circle *README.md* file for details!
* The *rustup* tool, for installing the Rust command line tools (e.g. cargo, rustc). See the [Rust documentation](https://www.rust-lang.org/) for details on installing Rust!

You have to add the the Rust targets *armv7a-none-eabi* (32-bit) and/or *aarch64-unknown-none* (64-bit) with the following commands:

	rustup target add armv7a-none-eabi
	rustup target add aarch64-unknown-none

The Raspberry Pi 1 and Zero are currently not supported, because Rust does not provide a bare-metal target for their ARMv6 CPU.

Installation
------------

Clone this project and the required submodules using:

	git clone https://github.com/rsta2/circle-rust.git
	cd circle-rust
	git submodule update --init circle bindgen

Build bindgen using:

	cd bindgen
	cargo build --release

rust-bindgen requires Clang / LLVM. See the [bindgen Users Guide](https://rust-lang.github.io/rust-bindgen/requirements.html) for details!

Configuration
-------------

Configure Circle as described in the Circle *README.md* file, e.g.:

	cd circle
	./configure -r 4 -p aarch64-none-elf-

Building
--------

The Circle project:

	cd circle
	./makeall clean
	./makeall -j

The wrapper library:

	cd lib/wrapper
	make clean
	make

The hello-world sample:

	cd sample/hello-world
	make
