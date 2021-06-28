mod masterkey;

fn main() {
    let mk_plain = masterkey::gen_masterkey();
    println!("masterkey is {:?}", mk_plain);
}
