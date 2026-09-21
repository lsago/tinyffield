use crate::{field::Field, group::Group, monoid::Monoid, ring::Ring};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GF2Field(bool);

impl GF2Field {
    pub fn new(value: bool) -> Self {
        Self(value)
    }
}

impl Monoid for GF2Field {
    fn zero() -> Self {
        Self(false)
    }
}

impl Group for GF2Field {}

impl Ring for GF2Field {
    fn one() -> Self {
        Self(true)
    }
}

impl Field for GF2Field {
    fn inverse(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            Some(Self(true))
        }
    }
}

impl std::ops::Add for GF2Field {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Neg for GF2Field {
    type Output = Self;

    fn neg(self) -> Self {
        self
    }
}

impl std::ops::Sub for GF2Field {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl std::ops::Mul for GF2Field {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 && rhs.0)
    }
}

impl std::fmt::Display for GF2Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn field_laws() {
    let elements = [GF2Field(false), GF2Field(true)];

    Field::assert_laws(&elements);
}
