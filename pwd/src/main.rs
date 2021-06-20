mod pwd;

fn main() {
    let result = pwd::get_encpwd("chomsky".to_string()).unwrap();

    println!("encrypted password is {}", result);
}
