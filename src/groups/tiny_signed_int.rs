use crate::group::Group;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TinySignedInt(i8);

impl Group for TinySignedInt {
    fn identity() -> Self {
        Self(0)
    }
}

impl std::ops::Add for TinySignedInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }
}

impl std::ops::Neg for TinySignedInt {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }
}

impl std::ops::Sub for TinySignedInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

// just show value (i8) when being asked to display it
impl std::fmt::Display for TinySignedInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[test]
fn add_two_elems() {
    let a = TinySignedInt(4);
    let b = TinySignedInt(5);

    assert_eq!(a + b, TinySignedInt(9));
}

#[test]
fn identity_does_nothing_example() {
    let iden = TinySignedInt::identity();
    let a = TinySignedInt(5);

    assert_eq!(a + iden, a);
}


#[test]
fn inverse_end() {
    let ident = TinySignedInt::identity();
    let a = TinySignedInt(-128);
    // the inverse of -128 for our group is not +128, but -128
    // because in i8 with wrapping: -128 + -128 = 0
    let inverse = -a;

    assert_eq!(a + inverse, ident);
}
