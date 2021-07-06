mod masterkey;
mod util;

use hex;

fn main() {
    let mk_enc   = masterkey::encrypt_masterkey("e1b7864680360cc7ad87c9e36c990f1ca43e577dd29b78f5a0cb34bd5ad27f5a".to_string());
    let mk = hex::encode(mk_enc);
    println!("(hex) encrypted masterkey is \"{}\"", mk);
}
