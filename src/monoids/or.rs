use crate::monoid::Monoid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrMonoid(u128);

impl OrMonoid {
    pub fn new(value: u128) -> Self {
        Self(value)
    }
}

impl Monoid for OrMonoid {
    fn zero() -> Self {
        Self(0)
    }
}

// op
impl std::ops::Add for OrMonoid {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

// just show value (u128) when being asked to display it
impl std::fmt::Display for OrMonoid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

#[test]
fn op_two_elems() {
    let a = OrMonoid(0b10100101010101011010101010011000100101u128);
    let b = OrMonoid(0b00110101000111011010101010001000100010u128);

    assert_eq!(
        a + b,
        OrMonoid(0b10110101010111011010101010011000100111u128)
    );
}

#[test]
fn monoid_laws() {
    let elements = [
        OrMonoid(0),
        OrMonoid(1),
        OrMonoid(0b10101010101000100101),
        OrMonoid(u128::MAX),
    ];

    OrMonoid::assert_laws(&elements);
}
