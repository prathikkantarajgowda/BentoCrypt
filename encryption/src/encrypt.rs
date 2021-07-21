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

use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead};

use no_std_compat::string::String;
use no_std_compat::vec::Vec;

use generic_array::{GenericArray, typenum::U12};

pub fn encrypt_data(data: &[u8], mk_str: String) -> Vec<u8> {

    /* print length of data (debugging) */
    //println!("length of data is {}", data.len());

    /* create cipher from masterkey */
    let mk_vec = hex::decode(mk_str).unwrap();
    let mk     = GenericArray::from_slice(mk_vec.as_slice());
    let cipher = Aes256Gcm::new(mk);

    /* generate nonce */
    let nonce  = util::gen_nonce();
    let nonce: &GenericArray<_, U12>  = GenericArray::from_slice(&nonce);

    /* 
     * encrypt our data using our cipher and nonce 
     * then check its validity using decrypt and assert_eq!
     */
    let mut enc_data = cipher.encrypt(nonce, data.as_ref())
        .expect("encryption failure!");
    let dec_data     = cipher.decrypt(nonce, enc_data.as_ref())
        .expect("decryption failure!");
    assert_eq!(&dec_data, data);

    /* remove the 16-byte authentication data, then return */
    enc_data.resize(enc_data.len() - 16, 0);
    return enc_data;
}
