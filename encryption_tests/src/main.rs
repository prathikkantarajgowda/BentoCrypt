fn main() {
    test1();
    test2();
    test3();
}


fn test1() {
    println!("\ntest 1");

    /* masterkey */
    let mk = "an example very very secret key.".to_string();
    println!("masterkey:  {}", mk);

    /* plaintext */
    let plaintext: [u8; 32] = [0u8; 32];
    let plaintext_hex       = hex::encode(plaintext);
    println!("plaintext:  {}", plaintext_hex);

    /* encryption */
    let ciphertext = encryption::encrypt_data(&plaintext, mk, "unique nonce".to_string());
    let ciphertext_hex = hex::encode(ciphertext);
    println!("ciphertext: {}", ciphertext_hex);
    let ciphertext = hex::decode(ciphertext_hex).unwrap();

    let mk = "an example very very secret key.".to_string();

    /* decryption */
    let decrypted = encryption::decrypt_data(ciphertext, mk, "unique nonce".to_string());
    let decrypted_hex = hex::encode(decrypted);
    println!("plaintext:  {}", decrypted_hex);
}


fn test2() {
    println!("\ntest 3");

    /* masterkey */
    let mk = "an example very very secret key.".to_string();
    println!("\nmasterkey:  {}", mk);

    /* plaintext */
    let plaintext: [u8; 64] = [0u8; 64];
    let plaintext_hex       = hex::encode(plaintext);
    println!("plaintext:  {}", plaintext_hex);

    /* encryption */
    let ciphertext = encryption::encrypt_data(&plaintext, mk, "unique nonce".to_string());
    let ciphertext_hex = hex::encode(ciphertext);
    println!("ciphertext: {}", ciphertext_hex);
    let ciphertext = hex::decode(ciphertext_hex).unwrap();

    let mk = "an example very very secret key.".to_string();

    /* decryption */
    let decrypted = encryption::decrypt_data(ciphertext, mk, "unique nonce".to_string());
    let decrypted_hex = hex::encode(decrypted);
    println!("plaintext:  {}", decrypted_hex);
}

fn test3() {
    println!("\ntest 2");

    /* masterkey */
    let mk = "an example very very secret key.".to_string();
    println!("\nmasterkey:  {}", mk);

    /* plaintext */
    let plaintext: [u8; 64] = [1u8; 64];
    let plaintext_hex       = hex::encode(plaintext);
    println!("plaintext:  {}", plaintext_hex);

    /* encryption */
    let ciphertext = encryption::encrypt_data(&plaintext, mk, "smelly jerry".to_string());
    let ciphertext_hex = hex::encode(ciphertext);
    println!("ciphertext: {}", ciphertext_hex);
    let ciphertext = hex::decode(ciphertext_hex).unwrap();

    let mk = "an example very very secret key.".to_string();

    /* decryption */
    let decrypted = encryption::decrypt_data(ciphertext, mk, "smelly jerry".to_string());
    let decrypted_hex = hex::encode(decrypted);
    println!("plaintext:  {}", decrypted_hex);
}
