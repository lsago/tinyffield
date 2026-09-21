use std::ops::Add;

/// A monoid is a set with an operation that combines any two elements of the
/// set to produce a third element within the same set (closure). That operation must
/// be associative. An identity element must also exist under this operation.
/// We are going to call the operation "Add" just so we can use
/// "+", but it could be anything.
///
/// A rust type implementing any of these traits does not force a the
/// mathematical structure upon it. For example, a type implementing the Group
/// trait does not necessarily mean any two instances of Group will satisfy the
/// group laws.
pub trait Monoid:
    Sized
    // binary operation
    + Add<Output = Self> {
    // identity element exists
    fn zero() -> Self;

    #[cfg(test)]
    fn assert_laws(elements: &[Self])
    where
        Self: Copy + Eq + std::fmt::Debug,
    {
        assert_monoid_laws(elements);
    }
}

// generic tests
#[cfg(test)]
pub(crate) fn assert_monoid_laws<M>(elements: &[M])
where
    M: Monoid + Copy + Eq + std::fmt::Debug,
{
    let identity = M::zero();

    for &e in elements {
        // identity does nothing
        assert_eq!(
            identity + e,
            e,
            "Identity rule \"identity + e\" failed: {:?} + {:?}",
            identity,
            e,
        );
        assert_eq!(
            e + identity,
            e,
            "Identity rule \"e + identity\" failed: {:?} + {:?}",
            e,
            identity,
        );
    }

    for &a in elements {
        for &b in elements {
            for &c in elements {
                // associativity
                assert_eq!(
                    (a + b) + c,
                    a + (b + c),
                    "Associativity rule \"(a + b) + c = a + (b + c)\" failed: a = {:?}, b = {:?}, c = {:?}",
                    a,
                    b,
                    c,
                );
            }
        }
    }
}
