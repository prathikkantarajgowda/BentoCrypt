mod pwd;

fn main() {
    let username = pwd::getlogin_rust()
        .expect("getlogin_rust failed");
    println!("login username is {}", username);

    let result = pwd::get_encpwd("prathik".to_string())
        .expect("get_encpwd failed");
    println!("encrypted password is {}", result);

    let kek = pwd::username_to_kek()
        .expect("username_to_kek failed");
    println!("kek is {}", kek);
}
