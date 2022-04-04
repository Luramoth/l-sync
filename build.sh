#!/bin/bash
# l-sync a content creator syncronisation tool
# Copyright (C) 2022  Luramoth

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
else
	cargo build $2
fi
