use tinyffield::groups::tiny_signed_int::TinySignedInt;
use tinyffield::groups::xor::Xor;

fn main() {
    let tiny_signed_a = TinySignedInt::new(4);
    let tiny_signed_b = TinySignedInt::new(5);
    println!("Group: tiny signed int:");
    println!("\tResult of {} ⊕ {} = {}", tiny_signed_a, tiny_signed_b, tiny_signed_a + tiny_signed_b);

    let xor_a = Xor::new(0b010100100100110101001u128);
    let xor_b = Xor::new(0b101110110110100101011u128);
    println!("Group: XOR");
    println!("\t Result of {} ⊕ {} = {}", xor_a, xor_b, xor_a + xor_b);
}
