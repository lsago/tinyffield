use crate::group::Group;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Xor(u128);

impl Xor {
    pub fn new(value: u128) -> Self {
        Self(value)
    }
}

impl Group for Xor {
    fn zero() -> Self {
        Self(0)
    }
}

// op
impl std::ops::Add for Xor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}


// inverse
impl std::ops::Neg for Xor {
    type Output = Self;

    fn neg(self) -> Self {
        // a ^ a = 0
        self
    }
}

impl std::ops::Sub for Xor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

// just show value (i8) when being asked to display it
impl std::fmt::Display for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[test]
fn op_two_elems() {
    let a = Xor(0b10100101010101011010101010011000100101u128);
    let b = Xor(0b00110101000111011010101010001000100010u128);

    assert_eq!(a + b, Xor(0b10010000010010000000000000010000000111u128));
}

#[test]
fn identity_does_nothing_example() {
    let iden = Xor::zero();
    let a = Xor(0b10010000010010000000000000010000000111u128);

    assert_eq!(a + iden, a);
}

#[test]
fn inverse_end() {
    let ident = Xor::zero();
    let max = Xor(u128::MAX);
    let inverse = -max;

    assert_eq!(max + inverse, ident);
}

#[test]
fn group_laws() {
    let elements = [
        Xor(0),
        Xor(1),
        Xor(0b10101010101000100101),
        Xor(u128::MAX),
    ];

    crate::group::assert_group_laws(&elements);
}
