#![cfg_attr(not(test), no_std)]

pub mod fraction;
pub mod float;

pub use fraction::FractionWheel;
pub use fraction::FractionWheel8;
pub use fraction::FractionWheel16;
pub use fraction::FractionWheel32;
pub use fraction::FractionWheel64;
pub use fraction::FractionWheel128;
pub use fraction::qw8;
pub use fraction::qw16;
pub use fraction::qw32;
pub use fraction::qw64;
pub use fraction::qw128;

pub use float::Wheel32;
pub use float::Wheel64;
pub use float::w32;
pub use float::w64;

/// Wheel is an algebraic structure where division is always defined.
/// Division is not necesarily the same as the multiplicative inverse.
/// Eq is always defined, but PartialOrd is not.
pub trait Wheel: PartialEq + Eq + Sized {
    /// Additive identity. There is no signed zero.
    const ZERO: Self;

    /// Multiplicative identity.
    const ONE: Self;

    /// Infinity is always unsigned.
    const INFINITY: Self;

    /// Bottom is a special value that represents an undefined value.
    /// But it is a number, so it can be used in calculations.
    const BOTTOM: Self;

    /// Normal addition.
    fn add(&self, other: &Self) -> Self;

    /// Normal negation.
    fn neg(&self) -> Self;

    /// Defined as `self + other.neg()`.
    /// `x - x` is not always zero.
    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// `0 * x` is not always zero.
    fn mul(&self, other: &Self) -> Self;

    /// Always defined. Not the same as the multiplicative inverse.
    fn inv(&self) -> Self;

    /// Always defined as `self * other.inv()`.
    /// `x / x` is not always one
    fn div(&self, other: &Self) -> Self {
        self.mul(&other.inv())
    }
}
