use crate::group::Group;

// represent internal value with an i8
type Value = i8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TinySignedIntElement {
    value: Value,
}

// just show value (i8) when being asked to display it
impl std::fmt::Display for TinySignedIntElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TinySignedIntElement {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}


// our group (TinySignedInts) made with a set of elements (TinySignedIntElement)
pub struct TinySignedInts;

impl Group for TinySignedInts {
    type Element = TinySignedIntElement;

    fn identity() -> Self::Element {
        TinySignedIntElement { value: 0 }
    }

    fn inverse(e: Self::Element) -> Self::Element {
        TinySignedIntElement { value: e.value.wrapping_neg() }
    }

    fn op(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        TinySignedIntElement {
            value: lhs.value.wrapping_add(rhs.value),
        }
    }
}

// The op is a property of the group structure, not the element. So doing
// TinySignedIntElement::new(3).op(...) would not make sense.
// But
impl std::ops::Add for TinySignedIntElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        <TinySignedInts as Group>::op(self, rhs)
    }
}

impl TinySignedInts {
    pub fn element(value: i8) -> TinySignedIntElement {
        TinySignedIntElement { value }
    }
}


#[test]
fn add_two_elems() {
    let a = TinySignedIntElement::new(4);
    let b = TinySignedIntElement::new(5);

    assert_eq!(a + b, TinySignedIntElement::new(9));
}

#[test]
fn identity_does_nothing_example() {
    let iden = TinySignedInts::identity();
    let a = TinySignedIntElement::new(5);

    assert_eq!(a + iden, a);
}


#[test]
fn inverse_end() {
    let ident = TinySignedInts::identity();
    let a = TinySignedIntElement::new(-128);
    // the inverse of -128 for our group is not +128, but -128
    // because in i8 with wrapping: -128 + -128 = 0
    let inverse = TinySignedInts::inverse(a);

    assert_eq!(a + inverse, ident);
}
