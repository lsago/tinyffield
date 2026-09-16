use tinyffield::rings::poly_gf2x7::PolyGF2x7;

fn main() {
    let a: PolyGF2x7 = "x^2 + 1".parse().unwrap();
    let b: PolyGF2x7 = "x^3 + 1".parse().unwrap();

    println!("Ring: GF(2)[x]/(x^7)");
    println!("({a}) * ({b}) = {}", a * b);
}
