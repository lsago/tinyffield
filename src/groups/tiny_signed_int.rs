use crate::group::Group;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TinySignedIntGroup(i8);

impl TinySignedIntGroup {
    pub fn new(value: i8) -> Self {
        Self(value)
    }
}

impl Group for TinySignedIntGroup {
    fn zero() -> Self {
        Self(0)
    }
}

impl std::ops::Add for TinySignedIntGroup {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }
}

impl std::ops::Neg for TinySignedIntGroup {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }
}

impl std::ops::Sub for TinySignedIntGroup {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

// just show value (i8) when being asked to display it
impl std::fmt::Display for TinySignedIntGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[test]
fn add_two_elems() {
    let a = TinySignedIntGroup(4);
    let b = TinySignedIntGroup(5);

    assert_eq!(a + b, TinySignedIntGroup(9));
}

#[test]
fn identity_does_nothing_example() {
    let iden = TinySignedIntGroup::zero();
    let a = TinySignedIntGroup(5);

    assert_eq!(a + iden, a);
}


#[test]
fn inverse_end() {
    let ident = TinySignedIntGroup::zero();
    let a = TinySignedIntGroup(-128);
    // the inverse of -128 for our group is not +128, but -128
    // because in i8 with wrapping: -128 + -128 = 0
    let inverse = -a;

    assert_eq!(a + inverse, ident);
}

#[test]
fn group_laws() {
    let elements = [
        TinySignedIntGroup(-128),
        TinySignedIntGroup(-1),
        TinySignedIntGroup(0),
        TinySignedIntGroup(1),
        TinySignedIntGroup(127),
    ];

    crate::group::assert_group_laws(&elements);
}
