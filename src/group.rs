/// "A group is a set with an operation that combines any two elements of the
/// set to produce a third element within the same set and the following
/// conditions must hold: the operation is associative, it has an identity
/// element, and every element of the set has an inverse element"
/// -- https://en.wikipedia.org/wiki/Group_(mathematics)
// we are going to call the operation of this group "Add", just so we can use
// "+". But it could be anything.
use std::ops::{Neg, Sub};

use crate::monoid::Monoid;

// Implementing this trait does not mean it's actually a group, just that you
// can use it as such if it actually is a group
pub trait Group:
    Sized
    + Monoid
    // inverse
    + Neg<Output = Self>
    // conveiniece: Add Neg
    + Sub<Output = Self> {
    #[cfg(test)]
    fn assert_laws(elements: &[Self])
    where
        Self: Copy + Eq + std::fmt::Debug,
    {
        assert_group_laws(elements);
    }
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
    };
}

// generic tests
#[cfg(test)]
pub(crate) fn assert_group_laws<G>(elements: &[G])
where
    G: Group + Copy + Eq + std::fmt::Debug,
{
    let identity = G::zero();

    // must also satisfy laws of monoids
    crate::monoid::assert_monoid_laws(&elements);

    for &e in elements {
        // inverse
        assert_eq!(
            -e + e,
            identity,
            "Inverse rule \"-e + e\" failed: {:?} + {:?}",
            -e,
            e,
        );
        assert_eq!(
            e + -e,
            identity,
            "Inverse rule \"e + -e\" failed: {:?} + {:?}",
            e,
            -e,
        );
    }
}
