# 
# BentoCrypt: a stacked, encrypted Linux file system written in Rust with Bento
# Copyright (C) 2021  Prathik Gowda (gowdapra@grinnell.edu)
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
#
#
# Makefile
#

.POSIX:

build:
	rustup override set nightly-2021-04-27
	@cargo build -Z build-std=core,alloc --release --target=x86_64-unknown-none-linuxkernel

run: build
	@cargo run

clean:
	@cargo clean
	@rm -rf *.o
