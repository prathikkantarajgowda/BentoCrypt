use rand_core::{RngCore, OsRng};

pub const MASTERKEYLEN: usize = 32; /* 32 bytes = 256 bits */
pub const NONCELEN:     usize = 12; /* 12 bytes = 96 bits  */

/*
 * gen_masterkey generates a random masterkey of length MASTERKEYLEN
 * (32 bytes; 256 bits for AES256-GCM encryption) using OsRng and returns it as
 * a byte array
 */
pub fn gen_masterkey() -> [u8; MASTERKEYLEN] {

    /* delare an array of bytes of MASTERKEYLEN length */
    let mut masterkey = [0u8; MASTERKEYLEN];

    /* fill the array with random bytes from the OS */
    OsRng.fill_bytes(&mut masterkey);

    return masterkey;
}

/*
 * gen_nonce generates a random nonce of length NONCELEN (12 bytes) using OsRng
 * and returns it as a byte array
 */
pub fn gen_nonce() -> [u8; NONCELEN] {

    /* declare an array of bytes of NONCELEN length */
    let mut nonce = [0u8; NONCELEN];

    /* fill the array with random bytes from the OS */
    OsRng.fill_bytes(&mut nonce);

    return nonce;
}
