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
 * util.rs
 */

use rand_core::{RngCore, OsRng};

/*
 * gen_masterkey generates a random masterkey of length MASTERKEYLEN
 * (32 bytes; 256 bits for AES256-GCM encryption) using OsRng and returns it as
 * a byte array
 */
pub fn gen_masterkey() -> [u8; 32] {

    /* delare an array of bytes of MASTERKEYLEN length */
    let mut masterkey = [0u8; 32];

    /* fill the array with random bytes from the OS */
    OsRng.fill_bytes(&mut masterkey);

    return masterkey;
}

/*
 * gen_nonce generates a random nonce of length NONCELEN (12 bytes) using OsRng
 * and returns it as a byte array
 */
pub fn gen_nonce() -> [u8; 12] {

    /* declare an array of bytes of NONCELEN length */
    let mut nonce = [0u8; 12];

    /* fill the array with random bytes from the OS */
    OsRng.fill_bytes(&mut nonce);

    return nonce;
}
