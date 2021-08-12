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
 * masterkey.rs
 */

use crate::util;

use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};

use hex;

use no_std_compat::string::String;
use no_std_compat::vec::Vec;

pub fn gen_enc_masterkey(kek_str: String) -> Vec<u8> {

    println!("\n======================================================================================");
    println!("|| BentoCrypt version 0.1, Copyright (C) 2021 Prathik Gowda <gowdapra@grinnell.edu> ||");
    println!("|| BentoCrypt comes with ABSOLUTELY NO WARRANTY                                     ||");
    println!("|| This is free software, and you are welcome to redistribute it                    ||");
    println!("|| under certain conditions.                                                        ||");
    println!("======================================================================================\n");

    println!("generating and encrypting masterkey...\n");

    /* decode (32-byte) kek from hex and then create a cipher out of it */
    let kek_vec   = hex::decode(kek_str)
        .unwrap();
    let kek       = GenericArray::from_slice(kek_vec.as_slice());
    let cipher    = Aes256Gcm::new(kek);

    /* generate (12-byte) nonce */
    let nonce = util::gen_nonce();
    let nonce = GenericArray::from_slice(&nonce);

    /* generate (32-byte) masterkey */
    let masterkey     = util::gen_masterkey();
    let masterkey_str = hex::encode(masterkey);

    /* finally, we can encrypt our masterkey */
    let ciphertext = cipher.encrypt(nonce, masterkey_str.as_ref())
        .expect("encryption failure!");

    /* now we verify our encrypted masterkey (panic on failure) */
    let plaintext  = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    assert_eq!(&plaintext, masterkey_str.as_bytes());

    println!("success!\n");

    return ciphertext;
}
