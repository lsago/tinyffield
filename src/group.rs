/// "A group is a set with an operation that combines any two elements of the
/// set to produce a third element within the same set and the following
/// conditions must hold: the operation is associative, it has an identity
/// element, and every element of the set has an inverse element"
/// -- https://en.wikipedia.org/wiki/Group_(mathematics)

// we are going to call the operation of this group "Add", just so we can use
// "+". But it could be anything.
use std::ops::{Add, Neg, Sub};

// Implementing this trait does not mean it's actually a group, just that you
// can use it as such if it actually is a group
pub trait Group:
    Sized
    // binary operation
    + Add<Output = Self>
    // inverse
    + Neg<Output = Self>
    // conveiniece: Add Neg
    + Sub<Output = Self> {
    // identity element exists
    fn identity() -> Self;
}

// Unfortunatelly, I'm not sure there's a way in Rust to provide this
// implementation by default for all types that implement this trait.
// It should be the same for all.
// impl std::ops::Sub for  {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self {
//         self + -rhs
//     }
// }
// So we'll settle for a macro..
#[macro_export]
macro_rules! impl_sub {
    // we require a type
    ($ty:ty) => {
        impl ::core::ops::Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                self + -rhs
            }
        }
    }
}

// generic tests
#[cfg(test)]
pub(crate) fn assert_group_laws<G>(elements: &[G])
where
    G: Group + Copy + Eq + std::fmt::Debug,
{
    let identity = G::identity();

    for &e in elements {
        // identity does nothing
        assert_eq!(
            identity + e, e,
            "Identity rule \"identity + e\" failed: {:?} + {:?}",
            identity, e,
        );
        assert_eq!(
            e + identity, e,
            "Identity rule \"e + identity\" failed: {:?} + {:?}",
            e, identity,
        );

        // inverse
        assert_eq!(
            -e + e, identity,
            "Inverse rule \"-e + e\" failed: {:?} + {:?}",
            -e, e,
        );
        assert_eq!(
            e + -e, identity,
            "Inverse rule \"e + -e\" failed: {:?} + {:?}",
            e, -e,
        );

    }

    for &a in elements {
        for &b in elements {
            for &c in elements {
                // associativity
                assert_eq!(
                    (a + b) + c, a + (b + c),
                    "Associativity rule \"(a + b) + c = a + (b + c)\" failed: a = {:?}, b = {:?}, c = {:?}",
                    a, b, c,
                );
            }
        }
    }
}
