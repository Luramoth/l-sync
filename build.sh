#!/bin/bash

echo "Bash version: $BASH_VERSION"

if [ "$1" = "--windows" ]; then
	rustup target add x86_64-pc-windows-gnu

	cargo build --target x86_64-pc-windows-gnu $2
elif [ "$1" = "--linux" ]; then
	rustup target add x86_64-unknown-linux-gnu

	cargo build --target x86_64-unknown-linux-gnu $2
elif [ "$1" = "--all" ]; then
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu

	cargo build --target x86_64-pc-windows-gnu $2
	cargo build --target x86_64-unknown-linux-gnu $2
fi