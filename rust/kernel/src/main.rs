mod masterkey;

fn main() {
    let mk_plain = masterkey::gen_masterkey();
    let nonce    = masterkey::gen_nonce();
    println!("masterkey is {:?}", mk_plain);

    let mk_enc   = masterkey::encrypt_masterkey(nonce, mk_plain, "e1b7864680360cc7ad87c9e36c990f1ca43e577dd29b78f5a0cb34bd5ad27f5a".to_string());
}
