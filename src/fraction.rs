//! Wheel implementation for fractions.

use crate::Wheel;

use core::ops::{Add, Sub, Mul, Div, Neg, Rem};
use core::fmt::Debug;

pub trait Ring: Add<Output=Self> + Mul<Output=Self> + Neg<Output=Self> + Copy + Clone + PartialEq + Eq + PartialOrd + Debug {
    const ZERO: Self;
    const ONE: Self;

    fn compare_pairs(a: (Self, Self), b: (Self, Self)) -> bool {
        let a0_is_zero = a.0 == Self::ZERO;
        let b0_is_zero = b.0 == Self::ZERO;
        let a1_is_zero = a.1 == Self::ZERO;
        let b1_is_zero = b.1 == Self::ZERO;
        match (a0_is_zero, b0_is_zero, a1_is_zero, b1_is_zero) {
            (true, true, false, false) => true,
            (false, false, true, true) => true,
            (true, true, true, true) => true,
            (false, false, false, false) => a.0 * b.1 == a.1 * b.0,
            _ => false,
        }
    }

    fn normalize_pair(pair: (Self, Self)) -> (Self, Self) {
        let first_is_zero = pair.0 == Self::ZERO;
        let second_is_zero = pair.1 == Self::ZERO;
        match (first_is_zero, second_is_zero) {
            (true, true) => (Self::ZERO, Self::ZERO),
            (true, false) => (Self::ZERO, Self::ONE),
            (false, true) => (Self::ONE, Self::ZERO),
            (false, false) => {
                (pair.0, pair.1)
            }
        }
    }
}

trait Gcd: Ring + Rem<Output=Self> + Ord {
    fn abs(&self) -> Self {
        if *self < Self::ZERO {
            -*self
        } else {
            *self
        }
    }

    fn gcd(a: Self, b: Self) -> Self {
        let mut a = a.abs();
        let mut b = b.abs();
        while b != Self::ZERO {
            let t = b;
            b = a % b;
            a = t;
        }
        if a == Self::ZERO {
            Self::ONE
        } else {
            a
        }
    }
}

impl Gcd for i8 {}
impl Gcd for i16 {}
impl Gcd for i32 {}
impl Gcd for i64 {}
impl Gcd for i128 {}

impl Ring for i8 {
    const ZERO: i8 = 0;
    const ONE: i8 = 1;

    fn normalize_pair((a, b): (Self, Self)) -> (Self, Self) {
        let gcd = Self::gcd(a, b);
        (a / gcd, b / gcd)
    }
}

impl Ring for i16 {
    const ZERO: i16 = 0;
    const ONE: i16 = 1;

    fn normalize_pair((a, b): (Self, Self)) -> (Self, Self) {
        let gcd = Self::gcd(a, b);
        (a / gcd, b / gcd)
    }
}

impl Ring for i32 {
    const ZERO: i32 = 0;
    const ONE: i32 = 1;

    fn normalize_pair((a, b): (Self, Self)) -> (Self, Self) {
        let gcd = Self::gcd(a, b);
        (a / gcd, b / gcd)
    }
}

impl Ring for i64 {
    const ZERO: i64 = 0;
    const ONE: i64 = 1;

    fn normalize_pair((a, b): (Self, Self)) -> (Self, Self) {
        let gcd = Self::gcd(a, b);
        (a / gcd, b / gcd)
    }
}

impl Ring for i128 {
    const ZERO: i128 = 0;
    const ONE: i128 = 1;

    fn normalize_pair((a, b): (Self, Self)) -> (Self, Self) {
        let gcd = Self::gcd(a, b);
        (a / gcd, b / gcd)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FractionWheel<T: Ring> (T, T);

impl<T: Ring> FractionWheel<T> {
    pub const ZERO: Self = FractionWheel(T::ZERO, T::ONE);
    pub const ONE: Self = FractionWheel(T::ONE, T::ONE);

    /// There is only one infinity (no signed infinity)
    pub const INFINITY: Self = FractionWheel(T::ONE, T::ZERO);

    /// 0/0
    pub const BOTTOM: Self = FractionWheel(T::ZERO, T::ZERO);

    pub fn new(numerator: T, denominator: T) -> Self {
        let value = FractionWheel(numerator, denominator);
        value.normalize()
    }

    fn normalize(&self) -> Self {
        let (numerator, denominator) = T::normalize_pair((self.0, self.1));
        if denominator < T::ZERO {
            FractionWheel(-numerator, -denominator)
        } else if denominator == T::ZERO && numerator < T::ZERO {
            FractionWheel(T::ONE, T::ZERO)
        } else {
            FractionWheel(numerator, denominator)
        }
    }

    fn add(&self, other: Self) -> Self {
        let a = self.0 * other.1;
        let b = self.1 * other.0;
        let c = self.1 * other.1;
        FractionWheel(a + b, c).normalize()
    }

    fn neg(&self) -> Self {
        FractionWheel(-self.0, self.1).normalize()
    }

    /// Defined as `self + other.neg()`.
    /// `x - x` is not always zero.
    fn sub(&self, other: Self) -> Self {
        self.add(other.neg())
    }

    /// `0 * x` is not always zero.
    fn mul(&self, other: Self) -> Self {
        let a = self.0 * other.0;
        let b = self.1 * other.1;
        FractionWheel(a, b).normalize()
    }

    /// Always defined. Not the same as the multiplicative inverse.
    pub fn inv(&self) -> Self {
        FractionWheel(self.1, self.0).normalize()
    }

    /// Always defined as `self * other.inv()`.
    /// `x / x` is not always one
    fn div(&self, other: Self) -> Self {
        self.mul(other.inv())
    }

    fn eq(&self, other: Self) -> bool {
        T::compare_pairs((self.0, self.1), (other.0, other.1))
    }
}

impl<T: Ring> Wheel for FractionWheel<T> {
    const ZERO: Self = FractionWheel::ZERO;
    const ONE: Self = FractionWheel::ONE;
    const INFINITY: Self = FractionWheel::INFINITY;
    const BOTTOM: Self = FractionWheel::BOTTOM;

    fn add(&self, other: &Self) -> Self {
        FractionWheel::add(self, *other)
    }

    fn neg(&self) -> Self {
        FractionWheel::neg(self)
    }

    fn mul(&self, other: &Self) -> Self {
        FractionWheel::mul(self, *other)
    }

    fn inv(&self) -> Self {
        FractionWheel::inv(self)
    }
}


// Conversion from integers

impl From<i8> for FractionWheel<i8> {
    fn from(value: i8) -> Self {
        FractionWheel(value, 1)
    }
}

impl From<i16> for FractionWheel<i16> {
    fn from(value: i16) -> Self {
        FractionWheel(value, 1)
    }
}

impl From<i32> for FractionWheel<i32> {
    fn from(value: i32) -> Self {
        FractionWheel(value, 1)
    }
}

impl From<i64> for FractionWheel<i64> {
    fn from(value: i64) -> Self {
        FractionWheel(value, 1)
    }
}

impl From<i128> for FractionWheel<i128> {
    fn from(value: i128) -> Self {
        FractionWheel(value, 1)
    }
}


// Arithmetic operators

// Add

impl<T: Ring> Add for FractionWheel<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(&self, other)
    }
}

impl<T: Ring> Add<&FractionWheel<T>> for FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn add(self, other: &Self) -> Self {
        Self::add(&self, *other)
    }
}

impl<T: Ring> Add<FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn add(self, other: FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::add(self, other)
    }
}

impl<T: Ring> Add<&FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn add(self, other: &FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::add(self, *other)
    }
}

// Sub

impl<T: Ring> Sub for FractionWheel<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::sub(&self, other)
    }
}

impl<T: Ring> Sub<&FractionWheel<T>> for FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn sub(self, other: &Self) -> Self {
        Self::sub(&self, *other)
    }
}

impl<T: Ring> Sub<FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn sub(self, other: FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::sub(self, other)
    }
}

impl<T: Ring> Sub<&FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn sub(self, other: &FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::sub(self, *other)
    }
}

// Mul

impl<T: Ring> Mul for FractionWheel<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::mul(&self, other)
    }
}

impl<T: Ring> Mul<&FractionWheel<T>> for FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn mul(self, other: &Self) -> Self {
        Self::mul(&self, *other)
    }
}

impl<T: Ring> Mul<FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn mul(self, other: FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::mul(self, other)
    }
}

impl<T: Ring> Mul<&FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn mul(self, other: &FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::mul(self, *other)
    }
}

// Div

impl<T: Ring> Div for FractionWheel<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::div(&self, other)
    }
}

impl<T: Ring> Div<&FractionWheel<T>> for FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn div(self, other: &Self) -> Self {
        Self::div(&self, *other)
    }
}

impl<T: Ring> Div<FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn div(self, other: FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::div(self, other)
    }
}

impl<T: Ring> Div<&FractionWheel<T>> for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn div(self, other: &FractionWheel<T>) -> FractionWheel<T> {
        FractionWheel::div(self, *other)
    }
}

// Neg

impl<T: Ring> Neg for FractionWheel<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(&self)
    }
}

impl<T: Ring> Neg for &FractionWheel<T> {
    type Output = FractionWheel<T>;

    fn neg(self) -> FractionWheel<T> {
        FractionWheel::neg(self)
    }
}


// Comparison operators

impl<T: Ring> PartialEq for FractionWheel<T> {
    fn eq(&self, other: &Self) -> bool {
        self.eq(*other)
    }
}

impl<T: Ring> Eq for FractionWheel<T> {}

pub type FractionWheel8 = FractionWheel<i8>;
pub type FractionWheel16 = FractionWheel<i16>;
pub type FractionWheel32 = FractionWheel<i32>;
pub type FractionWheel64 = FractionWheel<i64>;
pub type FractionWheel128 = FractionWheel<i128>;

pub use FractionWheel8 as qw8;
pub use FractionWheel16 as qw16;
pub use FractionWheel32 as qw32;
pub use FractionWheel64 as qw64;
pub use FractionWheel128 as qw128;


#[cfg(test)]
mod test {
    use super::*;
    type MyWheel = FractionWheel<i32>;

    const ZERO: MyWheel = MyWheel::ZERO;
    const ONE: MyWheel = MyWheel::ONE;
    const INFINITY: MyWheel = MyWheel::INFINITY;
    const BOTTOM: MyWheel = MyWheel::BOTTOM;

    #[inline]
    fn negative_one() -> MyWheel {
        -ONE
    }

    #[inline]
    fn three() -> MyWheel {
        ONE + ONE + ONE
    }

    #[inline]
    fn negative_two() -> MyWheel {
        -ONE - ONE
    }

    #[inline]
    fn three_halves() -> MyWheel {
        MyWheel::new(3, 2)
    }

    #[inline]
    fn negative_two_fifths() -> MyWheel {
        MyWheel::new(-2, 5)
    }

    #[inline]
    fn any_numbers() -> [MyWheel; 9] {
        [
            ZERO, ONE, INFINITY, BOTTOM,
            negative_one(), three(), negative_two(),
            three_halves(), negative_two_fifths()
        ]
    }

    #[test]
    fn inv_is_involution() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", x.inv().inv(), x);
            assert_eq!(x.inv().inv(), x);
        }
    }

    #[test]
    fn inv_is_multicative() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                println!("{:?} == {:?}", (x * y).inv(), y.inv() * x.inv());
                assert_eq!((x * y).inv(), y.inv() * x.inv());
            }
        }
    }

    /// `(x + y) * z + 0 * z = x * z + y * z`
    #[test]
    fn add_is_distributive() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                for &z in any_numbers().iter() {
                    println!("{:?} == {:?}", (x + y) * z + ZERO * z, x * z + y * z);
                    assert_eq!((x + y) * z + ZERO * z, x * z + y * z);
                }
            }
        }
    }

    /// `(x + y * z) / y = x / y + z + 0 * y`
    #[test]
    fn add_is_distributive_div() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                for &z in any_numbers().iter() {
                    println!("{:?} == {:?}", (x + y * z) / y, x / y + z + ZERO * y);
                    assert_eq!((x + y * z) / y, x / y + z + ZERO * y);
                }
            }
        }
    }

    /// `0 * 0 = 0`
    #[test]
    fn zero_times_zero() {
        assert_eq!(ZERO * ZERO, ZERO);
    }

    /// `(x + 0 * y) * z = x * z + 0 * y`
    #[test]
    fn zero_times_y() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                for &z in any_numbers().iter() {
                    println!("{:?} == {:?}", (x + ZERO * y) * z, x * z + ZERO * y);
                    assert_eq!((x + ZERO * y) * z, x * z + ZERO * y);
                }
            }
        }
    }

    /// `inv(x + 0 * y) = inv(x) + 0 * y`
    #[test]
    fn zero_times_y_inv() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                println!("{:?} == {:?}", (x + ZERO * y).inv(), x.inv() + ZERO * y);
                assert_eq!((x + ZERO * y).inv(), x.inv() + ZERO * y);
            }
        }
    }

    /// `0 / 0 + x = 0 / 0`
    #[test]
    fn bottom_addition() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", BOTTOM + x, BOTTOM);
            assert_eq!(BOTTOM + x, BOTTOM);
        }
    }

    /// `0 * x + 0 * y = 0 * x * y`
    #[test]
    fn zero_times_x_plus_zero_times_y() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                println!("{:?} == {:?}", ZERO * x + ZERO * y, ZERO * x * y);
                assert_eq!(ZERO * x + ZERO * y, ZERO * x * y);
            }
        }
    }

    /// `x / x = 1 + 0 * x / x`
    #[test]
    fn x_div_x() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", x / x, ONE + ZERO * x / x);
            assert_eq!(x / x, ONE + ZERO * x / x);
        }
    }

    /// `x - x = 0 * x * x`
    #[test]
    fn x_minus_x() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", x - x, ZERO * x * x);
            assert_eq!(x - x, ZERO * x * x);
        }
    }
}
