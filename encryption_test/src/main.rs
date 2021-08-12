use encryption::*;

fn main() {
    /* generate masterkey and encode as a hex string */
    let mk  = util::gen_masterkey();
    let mk  = hex::encode(mk);
    println!("\nmasterkey:  {}", mk);

    /* set up plaintext and print its contents */
    let data: [u8; 64] = [0u8; 64];
    let data_encoded = base64::encode(data);
    println!("plaintext:  {}", data_encoded);

    /* encrypt and print */
    let encrypted = encryption::encrypt_data(&data, mk.to_string(), "unique nonce".to_string());
    let encrypted_b64 = base64::encode(encrypted);
    println!("ciphertext: {}", encrypted_b64);
    let encrypted= base64::decode(encrypted_b64)
        .unwrap();

    /* decrypt and print */
    let decrypted = encryption::decrypt_data(encrypted, mk.to_string(), "unique nonce".to_string());
    let decrypted_encoded = base64::encode(decrypted);
    println!("plaintext:  {}", decrypted_encoded);
}
