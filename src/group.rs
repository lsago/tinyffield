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
