use super::{MAX_DEGREE, PolyGF2x7};
use crate::{fields::gf2::GF2Field, ring::Ring, rings::poly_gf2::parse};

impl std::str::FromStr for PolyGF2x7 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bin_coeffs = 0u8;

        for term in parse::parse(s) {
            let parsed_term = term?;

            // adding x^7 is a no op since we work modulo x^7
            if parsed_term.exp > MAX_DEGREE {
                continue;
            }
            bin_coeffs ^= u8::from(parsed_term.coeff == GF2Field::one()) << parsed_term.exp;
        }

        Ok(Self(bin_coeffs))
    }
}
