/// Polynomials in x, with coefficients from GF(2). Modulo x^7.
///   GF(2)[x]/(x^7)
/// Since x^7 is not irreducible, this is not a field.
use crate::{group::Group, monoid::Monoid, ring::Ring};

mod parse;

const MAX_DEGREE: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolyGF2x7(u8);

impl PolyGF2x7 {
    pub fn new(value: u8) -> Self {
        // maybe we should not allow values above 5
        Self(value)
    }
}

impl Monoid for PolyGF2x7 {
    fn zero() -> Self {
        Self(0)
    }
}

impl Group for PolyGF2x7 {}

impl Ring for PolyGF2x7 {
    fn one() -> Self {
        Self(1)
    }
}

// op
impl std::ops::Add for PolyGF2x7 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

// inverse for additive op
impl std::ops::Neg for PolyGF2x7 {
    type Output = Self;

    fn neg(self) -> Self {
        self
    }
}

impl std::ops::Sub for PolyGF2x7 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl std::ops::Mul for PolyGF2x7 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut bits = self.0;

        let mut total = 0u8;
        while bits != 0 {
            let i = bits.trailing_zeros();

            // in GF(2)[x], x + x = 0.
            // normal mul carries, so we can't just do self.0 * rhs.0
            total ^= rhs.0 << i;

            // set bit we are on to zero and bits to the right of it to 1
            // we knew it had all zeros (trailing_zeros), so and will be
            // clearing that bit effectively
            bits &= bits - 1;
        }

        // we only care about lower bits, higher represent exp over max degree,
        // which for us are no-ops, since we work modulo x^(MAX_DEGREE + 1)
        Self(total & ((1u8 << (MAX_DEGREE + 1)) - 1))
    }
}

// just show value (i8) when being asked to display it
impl std::fmt::Display for PolyGF2x7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bits = self.0;
        while bits != 0 {
            let i = u8::BITS - 1 - bits.leading_zeros();

            if i == 0 {
                write!(f, "1")?;
            } else if i == 1 {
                write!(f, "x")?;
            } else {
                write!(f, "x^{}", i)?;
            }
            // clear this bit
            bits &= !(1u8 << i);

            // if there are more terms coming, we print +
            if bits.trailing_ones() > 0 {
                write!(f, " + ")?;
            }
        }
        Ok(())
    }
}

#[test]
fn ring_laws() {
    use std::str::FromStr;

    let elements = [
        PolyGF2x7::from_str("0"),
        PolyGF2x7::from_str("x"),
        PolyGF2x7::from_str("x + 1"),
        PolyGF2x7::from_str("x^2 + 1"),
        PolyGF2x7::from_str("x^6 + x^5 + x^4 + x^3 + x^2 + x + 1"),
        PolyGF2x7::from_str("x^9 + 1"),
    ]
    .map(Result::unwrap);

    Ring::assert_laws(&elements);
}
