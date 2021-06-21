mod pwd;

fn main() {
    let result = pwd::get_encpwd("prathik".to_string())
        .expect("get_encpwd failed");

    println!("encrypted password is {}", result);

    let kek = pwd::derive_kek(result)
        .expect("derive_kek failed");

    println!("kek is {}", kek);
}
