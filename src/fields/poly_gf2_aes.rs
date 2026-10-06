use crate::{field::Field, group::Group, monoid::Monoid, ring::Ring, rings::poly_gf2::PolyGF2};

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
/// where r(x) is the reminder and deg(r) < deg(m) or r(x) = 0
/// p(x) = q(x) * m(x) + r(x)
///   p(x) + -p(x) = q(x) * m(x) + r(x) + (-p(x))
///   0 = q(x) * m(x) + r(x) + (-p(x))
///   In GF(2)[x], an element is its own additive inverse (-)
///   0 = q(x) * m(x) + r(x) + p(x)
/// So,
/// r(x) = q(x) * m(x) + p(x)
/// Since we are reducing m(x) where m(x) as deg(8), it means any reduced
/// element will be _lower_ than deg(8) and therefore fit in u8.
pub fn reduce(p: u16, m: u16) -> Option<u8> {
    let (_, r) = edivide(p, m)?;
    Some(r)
}

/// Euclidian division
/// P / M -> P = M*Q + R
/// Where R is either 0 or deg(R) < deg(M)
pub fn edivide(p: u16, m: u16) -> Option<(u16, u8)> {
    let mut q = 0u16;
    let mut p_i = p.clone();
    while deg(p_i) >= deg(m) {
        // Our goal is to match with M * Q the highest order bit in P.
        let q_i = p_i >> deg(m)?;

        // Our next step (after this one below) will be to
        // P_1 = P_0 - M * Q_0
        // We know we took care of the higest bit of P, but M*Q might still
        // result in exponentials higher than deg(M) but lower than deg(P)
        // So, if deg(P_1) < deg(M), we are done. But what if it's not?  That
        // means we need to adjust Q. We are going to accumulate the current Q
        // (Q_0) into `q`: $q = q_0 + q_1 + q_2 ...$,
        // Observe that deg(q_0) > deg(q_1) > deg(q_2) ...
        // So we know that next round (computing q_1 instead of q_0), when
        // computing the q_1 necessary to "take out" p_1 (P_1 = P_0 -
        // M * Q_0), the highest bit will be save - we are not going to flip it
        // again. And similarly for the rest of the rounds.
        // So, adding (XOR) works.
        q ^= q_i;

        // P_{i+1} = P_{i} - M * Q_{i}
        // In GF(2), adding (^) is the same as subtracting.
        p_i = poly_ring_mul(q_i, m) ^ p_i;
    }

    // For our particular irreducible m of degree 8, we know that any elemenent
    // modulo it, must have deg lower than 8, so `as u8` works
    Some((q, (poly_ring_mul(q, m) ^ p) as u8))
}

/// Single-coefficient extended Euclidian algorighm
pub fn single_extended_euclidian(r0: u16, r1: u16) -> Option<u16> {
    // Note: again, in GF(2) adding is the same as subtracting
    //
    // ediv(r0, r1) = (q, r2)  -> r0 = q*r1 + r2  ->  r2 = q*r1 + r0
    // ediv(r1, r2) = (q, r3)  -> r1 = q*r2 + r3  ->  r3 = q*r2 + r1
    // ediv(r2, r3) = (q, r4)  -> r2 = q*r3 + r4  ->  r4 = q*r3 + r2
    // ...
    // Since r1 was an irreducible polynomial, we'll reach an r_{n} = 1.
    // ediv(r_{n-2}, r_{n-1}) = (q, r_{n})
    //   -> r_{n-2} = q*r_{n-1} + r_{n}
    //   ->  r_{n} = q*r{n-1} + r{n-2}
    //   ->      1 = q*r{n-1} + r{n-2}
    // That "q*r{n-1} + r{n-2}" is a combination of r0 and r1 and whatever qs we
    // found along the way.
    // So.. we'd be able to express it with something like this:
    // `r0 * A + r1 * B = 1`
    // We don't really care about r1 * B, since r1 is our modulus, r1 * B
    // reduces to 0, so it's like adding the additive identity, it does nothing.
    // We ignore it - hence our _single_ extended euclidian algorithm name.
    // So we are left with:
    // `r0 * A = 1`
    // This (A) is the definition of the multiplicative inverse of r0.

    // r0 has 1 r0, r1 has 0 r0s and r2 (r0s_next) will
    // be computed below: r2 = r0 + r1*q0
    let mut r0s_prev: u16 = 1;
    let mut r0s_cur: u16 = 0;
    let mut r0s_next: u16;

    let mut r_prev: u16 = r0;
    let mut r_cur: u16 = r1;
    let mut r_next: u16;

    while r_cur != 1 {
        let (q, r_next_u8) = edivide(r_prev, r_cur)?;
        r_next = u16::from(r_next_u8);

        r0s_next = r0s_prev ^ poly_ring_mul(q, r0s_cur);
        r0s_prev = r0s_cur;
        r0s_cur = r0s_next;
        r_prev = r_cur;
        r_cur = r_next;
    }

    Some(r0s_cur)
}

impl PolyGF2AES {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn new_from_poly_str(s: &str) -> Result<Self, &'static str> {
        let polygf2: PolyGF2 = s.parse()?;
        // PolyGF2 is a _ring_. Coeffs are in GF(2), but the whole thing (poly)
        // is a ring.

        let Some(polygf2_deg) = polygf2.degree() else {
            return Ok(Self::zero());
        };

        if polygf2_deg >= u16::BITS as usize {
            return Err(
                "Reducing input polynomials of degree 15 or higher is not curretly implemented",
            );
        }

        Ok(Self(
            reduce(polygf2.lsb_u16(), MODULUS).ok_or("Error reducing")?,
        ))
    }

    pub fn sbox(self) -> Self {
        // Section 5.2
        let b = self.inverse().map_or_else(|| 0, |v| v.0);

        // Section 5.3, affine transformation
        Self(
            b ^ b.rotate_right(4)
                ^ b.rotate_right(5)
                ^ b.rotate_right(6)
                ^ b.rotate_right(7)
                ^ 0b01100011,
        )
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
    // We keep doing euclidian division until the reminder is 1, which it will be
    // as long as our polynomials are coprime. Since AES field is defined by an
    // irreducible (think prime) polynomial, this holds.
    fn inverse(self) -> Option<Self> {
        let coefficient = single_extended_euclidian(u16::from(self.0), MODULUS)?;
        Some(Self(coefficient as u8))
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

        Self(reduce(p, MODULUS).unwrap())
    }
}

impl std::fmt::Display for PolyGF2AES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

impl std::str::FromStr for PolyGF2AES {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u8::from_str_radix(s, 16)
            .map(Self)
            .map_err(|_| "invalid hex byte")
    }
}

#[test]
fn field_laws() {
    let elements = [
        PolyGF2AES::new_from_poly_str("0"),
        PolyGF2AES::new_from_poly_str("x"),
        PolyGF2AES::new_from_poly_str("x + 1"),
        PolyGF2AES::new_from_poly_str("x^2 + 1"),
        PolyGF2AES::new_from_poly_str("x^6 + x^5 + x^4 + x^3 + x^2 + x + 1"),
        Ok(PolyGF2AES::new(0xff)),
        Ok(PolyGF2AES::new(0xf0)),
        Ok(PolyGF2AES::new(0xde)),
        Ok(PolyGF2AES::new(0xad)),
        Ok(PolyGF2AES::new(0xbe)),
        Ok(PolyGF2AES::new(0xef)),
    ]
    .map(Result::unwrap);

    Field::assert_laws(&elements);
}
