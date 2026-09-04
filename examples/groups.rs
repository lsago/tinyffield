use tinyffield::groups::tiny_signed_int::TinySignedIntElement;

fn main() {
    let a = TinySignedIntElement::new(4);
    let b = TinySignedIntElement::new(5);

    println!("Result of {} + {} = {}", a, b, a + b);
}
