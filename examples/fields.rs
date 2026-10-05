use std::ops::Shl;

use tinyffield::{field::Field, fields::poly_gf2_aes::PolyGF2AES};

fn main() -> Result<(), &'static str> {
    aes()
}

fn aes() -> Result<(), &'static str> {
    println!("AES");

    println!("Addition example:");
    println!(
        "  This example is from section 4.2 of https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197-upd1.pdf"
    );
    let a = PolyGF2AES::new(0x57);
    let b = PolyGF2AES::new(0x83);
    let c = PolyGF2AES::new(0xd4);
    assert_eq!(a + b, c);
    println!("    {a} + {b} = {c}");

    let mut rcon_msb = PolyGF2AES::new(1);
    let x = PolyGF2AES::new_from_poly_str("x")?;

    println!(
        "During key expansation, AES uses a 4-byte word. This is the left-most byte, according to section 5.2"
    );
    for j in 1..=10 {
        println!("  Rcon_{j}[0] = {rcon_msb}");
        rcon_msb = rcon_msb * x;
    }

    let zero = PolyGF2AES::new(0x0);
    println!("Inverses are used in AES");
    println!(
        "Multiplicative inverse of {} is {}",
        zero,
        zero.inverse()
            .map_or_else(|| "undefined".to_string(), |v| v.to_string())
    );

    println!("Sbox - generating table 4 from section 5.1.1.");
    println!("     0   1   2   3   4   5   6   7   8   9   a   b   c   d   e   f");
    println!("   |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for x in 0..=0xfu8 {
        print!("|{x:2x}|");
        for y in 0..=0xfu8 {
            let v = PolyGF2AES::new(x.shl(4) | y).sbox();
            print!(" {v} ");
        }
        print!("\n");
    }

    Ok(())
}
