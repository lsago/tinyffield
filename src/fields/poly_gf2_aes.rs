use crate::{field::Field, group::Group, monoid::Monoid, ring::Ring};

/// The AES field GF(2^8), using the modulus x^8 + x^4 + x^3 + x + 1.
/// Bit i represents the coefficient of x^i.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolyGF2AES(u8);

impl PolyGF2AES {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl Monoid for PolyGF2AES {
    fn zero() -> Self {
        todo!()
    }
}

impl Group for PolyGF2AES {}

impl Ring for PolyGF2AES {
    fn one() -> Self {
        todo!()
    }
}

impl Field for PolyGF2AES {
    fn inverse(self) -> Option<Self> {
        todo!()
    }
}

impl std::ops::Add for PolyGF2AES {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        todo!()
    }
}

impl std::ops::Neg for PolyGF2AES {
    type Output = Self;

    fn neg(self) -> Self {
        todo!()
    }
}

impl std::ops::Sub for PolyGF2AES {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        todo!()
    }
}

impl std::ops::Mul for PolyGF2AES {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        todo!()
    }
}

impl std::fmt::Display for PolyGF2AES {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::str::FromStr for PolyGF2AES {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
