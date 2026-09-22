use std::ops::Mul;

use crate::group::Group;

pub trait Ring: Group + Mul<Output = Self> {
    // multiplicative identity
    fn one() -> Self;
    // additive identity we inherit from Group: zero()
    // group don't require a multiplicative inverse

    #[cfg(test)]
    fn assert_laws(elements: &[Self])
    where
        Self: Clone + Eq + std::fmt::Debug,
    {
        assert_ring_laws(elements);
    }
}

// generic tests
#[cfg(test)]
pub(crate) fn assert_ring_laws<R>(elements: &[R])
where
    R: Ring + Clone + Eq + std::fmt::Debug,
{
    let identity = R::one();

    // must be a group under +
    crate::group::assert_group_laws(&elements);

    for e in elements {
        // identity does nothing
        assert_eq!(
            identity.clone() * e.clone(),
            *e,
            "Identity rule \"identity * e\" failed: {:?} * {:?}",
            identity,
            e,
        );
        assert_eq!(
            e.clone() * identity.clone(),
            *e,
            "Identity rule \"e * identity\" failed: {:?} * {:?}",
            e,
            identity,
        );
    }

    for a in elements {
        for b in elements {
            // commutative group under addition (abelian group)
            assert_eq!(
                a.clone() + b.clone(),
                b.clone() + a.clone(),
                "Commutativity rule \"a + b = b + a\" failed: a = {:?}, b = {:?}",
                a,
                b,
            );

            for c in elements {
                // associativity for multiplication
                assert_eq!(
                    (a.clone() * b.clone()) * c.clone(),
                    a.clone() * (b.clone() * c.clone()),
                    "Associativity rule \"(a * b) * c = a * (b * c)\" failed: a = {:?}, b = {:?}, c = {:?}",
                    a,
                    b,
                    c,
                );

                // multiplication distributes under addition
                assert_eq!(
                    a.clone() * (b.clone() + c.clone()),
                    (a.clone() * b.clone() + a.clone() * c.clone()),
                    "Left distributivity rule failed: a = {:?}, b = {:?}, c = {:?}",
                    a,
                    b,
                    c,
                );
                assert_eq!(
                    (a.clone() + b.clone()) * c.clone(),
                    (a.clone() * c.clone() + b.clone() * c.clone()),
                    "Right distributivity rule failed: a = {:?}, b = {:?}, c = {:?}",
                    a,
                    b,
                    c,
                );
            }
        }
    }
}
