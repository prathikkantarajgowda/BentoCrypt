use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use generic_array::{ArrayLength, GenericArray};
use rand_core::{RngCore, OsRng};

pub const MASTERKEYLEN: usize = 32; /* 32 bytes = 256 bits */
pub const NONCELEN:     usize = 12; /* 12 bytes = 96 bits  */

/*
 * gen_masterkey generates a random masterkey of length MASTERKEYLEN
 * (32 bytes; 256 bits for AES256-GCM encryption) using OsRng. returns it as
 * a byte array
 */
pub fn gen_masterkey() -> [u8; MASTERKEYLEN] {

    /* delare an array of bytes of MASTERKEYLEN length */
    let mut masterkey = [0u8; MASTERKEYLEN];

    /* fill the array with random bytes from the OS */
    OsRng.fill_bytes(&mut masterkey);

    return masterkey;
}

pub fn encrypt_masterkey(nonce: [u8; NONCELEN],
                         masterkey: [u8; MASTERKEYLEN],
                         kek: String) -> Vec<u8> {

    /* 
     * convert kek from String to bytes, then to Key type. finally create
     * a cipher out of this key
     */
    let key = GenericArray::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);

    let nonce = gen_nonce();
    let nonce = GenericArray::from_slice(&nonce);
    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");

    return ciphertext;
}

pub fn gen_nonce() -> [u8; NONCELEN] {

    let mut nonce = [0u8; NONCELEN];
    OsRng.fill_bytes(&mut nonce);
    println!("nonce is {:?}", nonce);

    return nonce;
}
