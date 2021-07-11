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
 * encrypt.rs
 */

use crate::util;

use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};

pub fn encrypt(data: &[u8], masterkey: [u8; 32]) -> &[u8] {

    /* create cipher from masterkey */
    let mk_ref = GenericArray::clone_from_slice(&masterkey);
    let cipher = Aes256Gcm::new(&mk_ref);

    /* generate (12-byte) nonce */
    let nonce = util::gen_nonce();
    let nonce = GenericArray::clone_from_slice(&nonce);

    return data;
}
