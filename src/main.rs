extern crate haikunator;

fn main() {
    let mut h = haikunator::Haikunator::default();

    println!("{:?}", h.token_chars);
    println!("{:?}", h.haikunate());

    h.token_chars = "3";
    println!("{:?}", h.token_chars);
}
