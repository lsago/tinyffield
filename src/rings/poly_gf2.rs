/// Polynomials in x, with coefficients from GF(2).
///   GF(2)[x]
/// This should actually be a non-finite ring.
use crate::{group::Group, monoid::Monoid, ring::Ring};

mod display;
pub(crate) mod parse;

/// We represent our bitfield with a vector of u64s. u64s containing MSB towards
/// the end of the vec, LSB at the beginning.
///
/// In memory (little endian):
///
/// ```text
///             HIGHER ADDRESSES
///                    ^
/// +---------+--------+------------------+
/// | Address | Chunk  | High bit -> low  |
/// +---------+--------+------------------+
/// | 0x100F  | vec[1] | x^127 ... x^120  |
/// | 0x100E  | vec[1] | x^119 ... x^112  |
/// | 0x100D  | vec[1] | x^111 ... x^104  |
/// | 0x100C  | vec[1] | x^103 ... x^96   |
/// | 0x100B  | vec[1] | x^95  ... x^88   |
/// | 0x100A  | vec[1] | x^87  ... x^80   |
/// | 0x1009  | vec[1] | x^79  ... x^72   |
/// | 0x1008  | vec[1] | x^71  ... x^64   |
/// +---------+--------+------------------+
/// | 0x1007  | vec[0] | x^63  ... x^56   |
/// | 0x1006  | vec[0] | x^55  ... x^48   |
/// | 0x1005  | vec[0] | x^47  ... x^40   |
/// | 0x1004  | vec[0] | x^39  ... x^32   |
/// | 0x1003  | vec[0] | x^31  ... x^24   |
/// | 0x1002  | vec[0] | x^23  ... x^16   |
/// | 0x1001  | vec[0] | x^15  ... x^8    |
/// | 0x1000  | vec[0] | x^7   ... x^0    |
/// +---------+--------+------------------+
///                    v
///              LOWER ADDRESSES
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyGF2(Vec<u64>);

impl PolyGF2 {
    pub fn new(value: Vec<u64>) -> Self {
        // maybe we should not allow values above 5
        Self(value)
    }

    /// The degree of a nonzero polynomial is the largest exponent whose
    /// coefficient is nonzero. The degree of the zero polynomial is undefined.
    /// Therefore, we return Some(degree) or None
    // Technically, if the ring is really non-finite, we shoulnd't return u64,
    // but I don't think anyone could be around to hear this tree fall. So, this
    // (“ought to be enough for anybody”©)[https://docs.kernel.org/arch/x86/x86_64/5level-paging.html]
    pub fn degree(&self) -> Option<usize> {
        // Note: LSB represents coeff_{0}*x^{0} = coeff_0.
        //       If the LSB is the higest bit set, we have a polynomial of degree zero.
        //       If no bits at all are set, we are dealing with the zero polynomial,
        //       and degree is not defined for it.
        // Note: We might have more u64s than we need to represent this polynomial.
        //       This means some potentially whole 0u64s to the right of the first set bit
        self.0.iter().enumerate().rev().find_map(|(i, val)| {
            if *val == 0 {
                None
            } else {
                // No risk if underflow, we skipped it was zero so a set
                // bit is gueranteed
                Some((u64::BITS - val.leading_zeros() - 1) as usize + u64::BITS as usize * i)
            }
        })
    }

    fn poly_mul(lhs: u64, rhs: u64) -> u128 {
        let mut bits: u64 = lhs;
        let mut total: u128 = 0u128;

        while bits != 0 {
            let i = bits.trailing_zeros();

            // in GF(2)[x], x + x = 0.
            // normal mul carries, so we can't just do self.0 * rhs.0
            total ^= (rhs as u128) << i;

            // set bit we are on to zero and bits to the right of it to 1
            // we knew it had all zeros (trailing_zeros), so and will be
            // clearing that bit effectively
            bits &= bits - 1;
        }

        total
    }

    pub fn lsb_u16(&self) -> u16 {
        self.0[0] as u16
    }
}

impl Monoid for PolyGF2 {
    fn zero() -> Self {
        Self(vec![0u64])
    }
}

impl Group for PolyGF2 {}

impl Ring for PolyGF2 {
    fn one() -> Self {
        Self(vec![1u64])
    }
}

// op
impl std::ops::Add for PolyGF2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(
            self.0
                .iter()
                .zip(rhs.0)
                .map(|(lhs, rhs)| lhs ^ rhs)
                .collect(),
        )
    }
}

// inverse for additive op
impl std::ops::Neg for PolyGF2 {
    type Output = Self;

    fn neg(self) -> Self {
        // Given GF(2), elements are their own inverse (xor)
        self
    }
}

impl std::ops::Sub for PolyGF2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl std::ops::Mul for PolyGF2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result_size = (self.degree().unwrap_or(0) + rhs.degree().unwrap_or(0) + 1)
            .div_ceil(u64::BITS as usize);

        let mut result = vec![0u64; result_size];

        for (lhs_i, lhs_u64) in self.0.iter().enumerate() {
            for (rhs_i, rhs_u64) in rhs.0.iter().enumerate() {
                // we might have u64s that are empty,
                // P(x + 1) * Q(x^66 + x^65) .. we don't have to care about Q's first u64
                // (x + 1) * (0x^63 + ... + 0x^2 + 0x + 0) = 0
                // We also don't care about u64s with all zeros to the right of the MSB
                // A bitfield might started with 5 u64s but if at some point we
                // set the two MSB u64s with zeros.. those u64s might still be in there
                // (Maybe, haven't decided if making the Vec immutable is what we want or not..)
                if *rhs_u64 == 0 || *lhs_u64 == 0 {
                    continue;
                }

                // e.g. x^3 from lhs combines with x^7 from rhs to produce x^10
                let result_i = lhs_i + rhs_i;

                let mul_res = PolyGF2::poly_mul(*lhs_u64, *rhs_u64);
                let mul_res_low = mul_res as u64;
                let mul_res_high = (mul_res >> u64::BITS) as u64;
                result[result_i] ^= mul_res_low;
                // we might not have allocated the space in our vec, as computed
                // bits/u64s needed using their degree but multiplying always
                // prodeces an u128, even if upper half is empty
                if mul_res_high != 0 {
                    result[result_i + 1] ^= mul_res_high;
                }
            }
        }
        Self(result)
    }
}

impl std::fmt::Display for PolyGF2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display::fmt(&self.0, f)
    }
}

#[test]
fn ring_laws() {
    use std::str::FromStr;

    let elements = [
        PolyGF2::from_str("0"),
        PolyGF2::from_str("x"),
        PolyGF2::from_str("x + 1"),
        PolyGF2::from_str("x^2 + 1"),
        PolyGF2::from_str("x^6 + x^5 + x^4 + x^3 + x^2 + x + 1"),
        PolyGF2::from_str("x^9 + 1"),
    ]
    .map(Result::unwrap);

    Ring::assert_laws(&elements);
}
