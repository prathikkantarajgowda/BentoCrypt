/* 
 * BentoCrypt: a stacked, encrypted Linux file system written in Rust with Bento
 * Copyright (C) 2021  Prathik Gowda (gowdapra@grinnell.edu)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 *
 * main.rs
 */

mod encrypt;
mod masterkey;
mod util;

fn main() {
    let kek = "b0e50692172d16a8d160675b6fb3dfe4b02158659f2041c66cb32b6055ba45db"
        .to_string();

    masterkey::gen_enc_masterkey(kek.to_string());

    let data: [u8; 64] = [0u8; 64];
    let _encrypted = encrypt::encrypt_data(&data, kek);
}
