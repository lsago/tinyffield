use crate::{field::Field, group::Group, monoid::Monoid, ring::Ring};

/// The AES field GF(2^8), using the modulus x^8 + x^4 + x^3 + x + 1.
/// Bit i represents the coefficient of x^i.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolyGF2AES(u8);

//       1 --------------------\
//       x -------------------\|
//     x^3 -----------------\ ||
//     x^4 ----------------\| ||
//     x^8 ------------\   || ||
//                     |   || ||
const MODULUS: u16 = 0b100011011;

fn deg(x: u16) -> Option<usize> {
    ((u16::BITS - x.leading_zeros()) as usize).checked_sub(1)
}

// We do multiplication in _ring_ GF(2)[x]. Multiplying there and reducing
// is the field multiplication.
// I could technically use the existing poly ring I have implemented already,
// but that one is for non-finite rings and I don't want to deal with Vecs now.
fn poly_ring_mul(a: u16, b: u16) -> u16 {
    // deg(a) + deg(b) < u16::BITS
    let mut bits = a;
    let mut c = 0u16;

    while bits != 0 {
        let i = bits.trailing_zeros();

        // in GF(2)[x], x + x = 0.
        // normal mul carries, so we can't just do a * b
        c ^= b << i;

        // set bit we are on to zero and bits to the right of it to 1
        // we knew it had all zeros (trailing_zeros), so and will be
        // clearing that bit effectively
        bits &= bits - 1;
    }
    c
}

/// Given a polynomial p(x) and m(x) in GF(2)[x] compute r(x)
/// where r(x) is the reminder and deg(r) < deg(m).
/// p(x) = q(x) * m(x) + r(x)
///   p(x) + (-p(x) = q(x) * m(x) + r(x) + (-p(x))
///   0 = q(x) * m(x) + r(x) + (-p(x))
///   In GF(2)[x], an element is its own additive inverse (-)
///   0 = q(x) * m(x) + r(x) + p(x)
/// So,
/// r(x) = q(x) * m(x) + p(x)
/// Given we are outputting a u8, we must have deg(m) <= 8
// TODO: LLM says there's a way to do this wihtout using q...
pub fn reduce(p: u16, m: u16) -> Option<u8> {
    // we'll start constructing q(x) in batches: q_1(x), q_2(x), etc..
    // at the end we'll have:
    // r(x) = (q_1(x) + q_2(x) + ... + q_n(x) ) * m(x) + p(x)
    let mut q = 0u16;
    let mut p_i = p.clone();
    while deg(p_i) >= deg(m) {
        let q_i = p_i >> deg(m)?;
        q ^= q_i;
        p_i = poly_ring_mul(q_i, m) ^ p_i;
    }
    Some((poly_ring_mul(q, m) ^ p) as u8)
}


impl PolyGF2AES {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl Monoid for PolyGF2AES {
    fn zero() -> Self {
        Self(0u8)
    }
}

impl Group for PolyGF2AES {}

impl Ring for PolyGF2AES {
    fn one() -> Self {
        Self(1u8)
    }
}

impl Field for PolyGF2AES {
    fn inverse(self) -> Option<Self> {
        todo!()
    }
}

impl std::ops::Add for PolyGF2AES {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Neg for PolyGF2AES {
    type Output = Self;

    fn neg(self) -> Self {
       self
    }
}

impl std::ops::Sub for PolyGF2AES {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl std::ops::Mul for PolyGF2AES {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut bits = self.0;
        let mut p = 0u16;

        while bits != 0 {
            let i = bits.trailing_zeros();

            // in GF(2)[x], x + x = 0.
            // normal mul carries, so we can't just do self.0 * rhs.0
            p ^= (rhs.0 as u16) << i;

            // set bit we are on to zero and bits to the right of it to 1
            // we knew it had all zeros (trailing_zeros), so and will be
            // clearing that bit effectively
            bits &= bits - 1;
        }

        // reduction:..
        // imagine p(x) = a(x) * b(x)
        // that * op alone is not the op of a field, closure is not even respected
        // but we keep going
        // we reduce it modulo something, like this
        // p(x) = q(x) * m(x) + r(x)
        // p(x) + (-p(x) = q(x) * m(x) + r(x) + (-p(x))
        // 0 = q(x) * m(x) + r(x) + (-p(x))
        // in GF(2)[x], an element is it's own additive inverse (-)
        // 0 = q(x) * m(x) + r(x) + p(x)
        // r(x) = q(x) * m(x) + p(x)
        // With the requirement that given that m(x), we choose a q(x) so that
        // the degree of r(x) is lower than the degree of m(x)
        // And... if we def our Mul op as yielding that r(x) .. we form a field.
        // r(x) and q(x) will be unique (that prove is trivial with deg)
        // I still have to prove all the other stuff to myself..
        Self(reduce(p, MODULUS).unwrap())
    }

}

impl std::fmt::Display for PolyGF2AES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::str::FromStr for PolyGF2AES {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
