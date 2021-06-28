mod pwd;
mod util;

extern crate base64;
use base64::{encode, decode};

fn main() {

    /* username_to_kek call */
    let kek = pwd::username_to_kek()
        .expect("username_to_kek failed");

    println!("kek is {}", kek);

    let decoded: Vec<u8> = base64::decode("LiEAw54nabV42seqP2aP/HJy+umdLpKV0EFNwpPddH4")
        .unwrap();

    println!("vector: {:?}", decoded);
}
