use crate::ring::Ring;

pub trait Field: Ring {
    // multiplicative inverse exists for all elements except add identitive
    fn inverse(self) -> Option<Self>;

    #[cfg(test)]
    fn assert_laws(elements: &[Self])
    where
        Self: Copy + Eq + std::fmt::Debug,
    {
        assert_field_laws(elements);
    }
}

// generic tests
#[cfg(test)]
pub(crate) fn assert_field_laws<F>(elements: &[F])
where
    F: Field + Copy + Eq + std::fmt::Debug,
{
    let add_identity = F::zero();
    let mul_identity = F::one();

    // must be a group under +
    crate::ring::assert_ring_laws(&elements);

    // 0 != 1
    assert_ne!(add_identity, mul_identity);

    for &e in elements {
        if e == add_identity {
            assert_eq!(e.inverse(), None);
        } else {
            let e_inverse = e
                .inverse()
                .expect("nonzero element must have a mul inverse");
            // mul inverses
            assert_eq!(
                e_inverse * e,
                mul_identity,
                "Inverse rule \"e^-1 * e\" failed: {:?} * {:?}",
                e_inverse,
                e,
            );
            assert_eq!(
                e * e_inverse,
                mul_identity,
                "Inverse rule \"e * e^-1\" failed: {:?} * {:?}",
                e,
                e_inverse,
            );
        }
    }

    for &a in elements {
        for &b in elements {
            // commutative for multiplication
            assert_eq!(
                a * b,
                b * a,
                "Commutativity rule \"a * b = b * a\" failed: a = {:?}, b = {:?}",
                a,
                b,
            );
        }
    }
}
