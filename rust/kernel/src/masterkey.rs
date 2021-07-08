use crate::util;

use aes_gcm::{Aes256Gcm, Key};
use aes_gcm::aead::{Aead, NewAead};
use generic_array::{GenericArray};
use hex;

pub fn encrypt_masterkey(kek_str: String) -> Vec<u8> {

    println!("\n======================================================================================");
    println!("|| BentoCrypt version 0.1, Copyright (C) 2021 Prathik Gowda <gowdapra@grinnell.edu> ||");
    println!("|| BentoCrypt comes with ABSOLUTELY NO WARRANTY                                     ||");
    println!("|| This is free software, and you are welcome to redistribute it                    ||");
    println!("|| under certain conditions.                                                        ||");
    println!("======================================================================================\n");

    println!("generating and encrypting masterkey...\n");
    println!("\n(hex) kek is {} chars long", kek_str.chars().count());
    println!("(hex) kek is \"{}\"", kek_str);

    /* decode (32-byte) kek from hex and then create a cipher out of it */
    let kek_vec   = hex::decode(kek_str)
        .unwrap();
    let kek       = Key::from_slice(kek_vec.as_slice());
    let cipher    = Aes256Gcm::new(kek);

    /* generate (12-byte) nonce */
    let nonce = util::gen_nonce();
    let nonce = GenericArray::from_slice(&nonce);

    println!("\n(hex) nonce is {} chars long", (hex::encode(nonce)).chars().count());
    println!("(hex) nonce is \"{}\"", hex::encode(nonce));

    /* generate (32-byte) masterkey */
    let masterkey     = util::gen_masterkey();
    let masterkey_str = hex::encode(masterkey);

    println!("\n(hex) masterkey is {} chars long", masterkey_str.chars().count());
    println!("(hex) masterkey is \"{}\"", masterkey_str);

    let ciphertext = cipher.encrypt(nonce, masterkey_str.as_ref())
        .expect("encryption failure!");

    println!("\nencrypted masterkey is {} bytes long", ciphertext.len());

    return ciphertext;
}
