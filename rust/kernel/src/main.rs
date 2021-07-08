mod masterkey;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let kek = &args[1];

    println!("received kek is {}", kek);

    let mk_enc = masterkey::encrypt_masterkey(kek.to_string());
    let mk_enc_hex = hex::encode(mk_enc);
}
