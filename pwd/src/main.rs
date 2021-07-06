mod pwd;
mod util;

fn main() {

    /* username_to_kek call */
    let kek = pwd::username_to_kek()
        .expect("username_to_kek failed");

    println!("kek is {}", kek);
}
