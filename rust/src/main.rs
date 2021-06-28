mod masterkey;

fn main() {
    let mk_plain = masterkey::gen_masterkey();
    let nonce    = masterkey::gen_nonce();
    println!("masterkey is {:?}", mk_plain);

    let mk_enc   = masterkey::encrypt_masterkey(nonce, mk_plain, "12345678".to_string());
}
