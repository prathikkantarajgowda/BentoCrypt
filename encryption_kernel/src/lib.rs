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
 * lib.rs
 */

#![no_std]

use aes_gcm::{Aes256Gcm, Nonce, Key};
use aes_gcm::aead::{Aead, NewAead};

use generic_array::GenericArray;

use no_std_compat::string::String;
use no_std_compat::vec::Vec;

use rand_core::{RngCore, OsRng};

/* function to encrypt an array of bytes */
pub fn encrypt_data(data: &[u8], mk_str: String, nonce_str: String) -> Vec<u8> {

    /* create cipher from masterkey */
    let mk     = Key::from_slice(mk_str.as_bytes());
    let cipher = Aes256Gcm::new(mk);

    /* nonce */
    let nonce = Nonce::from_slice(nonce_str.as_bytes());

    /* 
     * encrypt our data using our cipher and nonce 
     * then check its validity using decrypt and assert_eq!
     */
    let enc_data = cipher.encrypt(nonce, data.as_ref())
        .expect("encryption failure!");
    let dec_data = cipher.decrypt(nonce, enc_data.as_ref())
        .expect("decryption failure!");
    assert_eq!(&dec_data, data);

    return enc_data;
}

/* function to decrypt an array of bytes (encrypted with AES-GCM) */
pub fn decrypt_data(data: Vec<u8>, mk_str: String, nonce: String) -> Vec<u8> {
    
    /* create cipher from masterkey */
    let mk     = Key::from_slice(mk_str.as_bytes());
    let cipher = Aes256Gcm::new(mk);

    /* nonce */
    let nonce  = Nonce::from_slice(nonce.as_bytes());

    /* decryption */
    let dec_data = cipher.decrypt(nonce, data.as_ref())
        .expect("decryption failure!");

    return dec_data;
}

/* function to generate an encrypted masterkey from a key encryption key */
pub fn gen_enc_masterkey(kek_str: String) -> Vec<u8> {

    /* decode (32-byte) kek from hex and then create a cipher out of it */
    let kek_vec   = hex::decode(kek_str)
        .unwrap();
    let kek       = GenericArray::from_slice(kek_vec.as_slice());
    let cipher    = Aes256Gcm::new(kek);

    /* generate (12-byte) nonce */
    let nonce = gen_nonce();
    let nonce = GenericArray::from_slice(&nonce);

    /* generate (32-byte) masterkey */
    let masterkey     = gen_masterkey();
    let masterkey_str = hex::encode(masterkey);

    /* finally, we can encrypt our masterkey */
    let ciphertext = cipher.encrypt(nonce, masterkey_str.as_ref())
        .expect("encryption failure!");

    /* now we verify our encrypted masterkey (panic on failure) */
    let plaintext  = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    assert_eq!(&plaintext, masterkey_str.as_bytes());

    return ciphertext;
}

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
