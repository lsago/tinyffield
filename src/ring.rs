use std::ops::Mul;

use crate::group::Group;


pub trait Ring:
    Group
    + Mul<Output = Self> {

    // multiplicative identity
    fn one() -> Self;
    // additive identity we inherit from Group: zero()
    // group don't require a multiplicative inverse
}

// generic tests
#[cfg(test)]
pub(crate) fn assert_ring_laws<R>(elements: &[R])
where
    R: Ring + Copy + Eq + std::fmt::Debug,
{
    let identity = R::one();

    // must be a group under +
    crate::group::assert_group_laws(&elements);

    for &e in elements {
        // identity does nothing
        assert_eq!(
            identity * e, e,
            "Identity rule \"identity * e\" failed: {:?} * {:?}",
            identity, e,
        );
        assert_eq!(
            e * identity, e,
            "Identity rule \"e * identity\" failed: {:?} * {:?}",
            e, identity,
        );
    }

    for &a in elements {
        for &b in elements {
            // commutative group under addition (abelian group)
            assert_eq!(
                a + b, b + a,
                "Commutativity rule \"a + b = b + a\" failed: a = {:?}, b = {:?}",
                a, b,
            );

            for &c in elements {
                // associativity for multiplication
                assert_eq!(
                    (a * b) * c, a * (b * c),
                    "Associativity rule \"(a * b) * c = a * (b * c)\" failed: a = {:?}, b = {:?}, c = {:?}",
                    a, b, c,
                );

                // multiplication distributes under addition
                assert_eq!(
                    a * (b + c), (a * b + a * c),
                    "Left distributivity rule failed: a = {:?}, b = {:?}, c = {:?}",
                    a, b, c,
                );
                assert_eq!(
                    (a + b) * c, (a * c + b * c),
                    "Right distributivity rule failed: a = {:?}, b = {:?}, c = {:?}",
                    a, b, c,
                );

            }
        }
    }
}
