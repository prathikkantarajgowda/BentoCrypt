use crate::util;

use aes_gcm::{Aes256Gcm, Key};
use aes_gcm::aead::{Aead, NewAead};
use generic_array::{GenericArray};
use hex;

pub fn encrypt_masterkey(kek_str: String) -> Vec<u8> {

    println!("\n(hex) kek is \"{}\"", kek_str);

    /* decode kek from hex and then create a cipher out of it */
    let kek_vec   = hex::decode(kek_str)
        .unwrap();
    let kek       = Key::from_slice(kek_vec.as_slice());
    let cipher    = Aes256Gcm::new(kek);

    /* generate nonce */
    let nonce = util::gen_nonce();
    let nonce = GenericArray::from_slice(&nonce);

    /* generate masterkey */
    let masterkey     = util::gen_masterkey();
    let masterkey_str = hex::encode(masterkey);

    println!("(hex) masterkey is \"{}\"", masterkey_str);

    let ciphertext = cipher.encrypt(nonce, masterkey_str.as_ref())
        .expect("encryption failure!");

    return ciphertext;
}
