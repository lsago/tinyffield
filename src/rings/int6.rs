use crate::{group::Group, ring::Ring};

const MODULO: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int6(u8);

impl Int6 {
    pub fn new(value: u8) -> Self {
        // maybe we should not allow values above 5
        Self(value % MODULO)
    }

}

impl Group for Int6 {
    fn zero() -> Self {
        Self(0)
    }
}

impl Ring for Int6 {
    fn one() -> Self {
        Self(1)
    }
}

// op
impl std::ops::Add for Int6 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) % MODULO)
    }
}


// inverse for additive op
impl std::ops::Neg for Int6 {
    type Output = Self;

    fn neg(self) -> Self {
        // self.0 is always < MODULO. But MODULO - 0 \is_not in our Int6
        Self((MODULO - self.0) % MODULO)
    }
}

impl std::ops::Sub for Int6 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl std::ops::Mul for Int6 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self((self.0 * rhs.0) % MODULO)
    }
}

// just show value (i8) when being asked to display it
impl std::fmt::Display for Int6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn ring_laws() {
    let elements = [
        Int6(0),
        Int6(1),
        Int6(2),
        Int6(3),
        Int6(4),
        Int6(5),
    ];

    crate::ring::assert_ring_laws(&elements);
}
