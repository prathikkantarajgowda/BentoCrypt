use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
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

pub fn encrypt_masterkey(masterkey: [u8; MASTERKEYLEN], kek: String) -> [u8; MASTERKEYLEN] {

    /* 
     * convert kek from String to bytes, then to Key type. finally create
     * a cipher out of this key
     */
    let kek_bytes: &[u8] = kek.as_bytes();
    let key              = Key::from_slice(kek_bytes);
    let cipher           = Aes256Gcm::new(key);

    /* create and fill a nonce (len = 12 bytes = 96-bits) with random bytes */
    let mut nonce = [0u8; NONCELEN];
    OsRng.fill_bytes(&mut nonce);
    let nonce = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce, masterkey.as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

    return masterkey;
}
