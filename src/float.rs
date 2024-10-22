
use crate::Wheel;

use core::ops::{Add, Sub, Mul, Div, Neg};
use core::num::FpCategory;
use core::fmt::{self, Display, Debug, Formatter};


#[derive(Clone, Copy)]
pub struct Wheel32(f32);
pub use Wheel32 as w32;

#[derive(Clone, Copy)]
pub struct Wheel64(f64);
pub use Wheel64 as w64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FpWheelCategory {
    Zero,
    Infinity,
    Bottom,
    Normal,
}

trait WheelCategoryGetter {
    fn get_category(&self) -> FpWheelCategory;
}

impl WheelCategoryGetter for f32 {
    #[inline]
    fn get_category(&self) -> FpWheelCategory {
        match self.classify() {
            FpCategory::Zero => FpWheelCategory::Zero,
            FpCategory::Infinite => FpWheelCategory::Infinity,
            FpCategory::Nan => FpWheelCategory::Bottom,
            FpCategory::Normal => FpWheelCategory::Normal,
            FpCategory::Subnormal => FpWheelCategory::Normal,
        }
    }
}

impl WheelCategoryGetter for f64 {
    #[inline]
    fn get_category(&self) -> FpWheelCategory {
        match self.classify() {
            FpCategory::Zero => FpWheelCategory::Zero,
            FpCategory::Infinite => FpWheelCategory::Infinity,
            FpCategory::Nan => FpWheelCategory::Bottom,
            FpCategory::Normal => FpWheelCategory::Normal,
            FpCategory::Subnormal => FpWheelCategory::Normal,
        }
    }
}


// Implementations for Wheel32

impl Wheel32 {
    pub const ZERO: Self = Wheel32(0.0);
    pub const ONE: Self = Wheel32(1.0);
    pub const NEGATIVE_ONE: Self = Wheel32(-1.0);
    pub const INFINITY: Self = Wheel32(f32::INFINITY);
    pub const BOTTOM: Self = Wheel32(f32::NAN);

    pub fn new(value: f32) -> Self {
        Wheel32(value)
    }

    fn eq(&self, other: Self) -> bool {
        let self_category = self.0.get_category();
        let other_category = other.0.get_category();
        if self_category != other_category {
            return false;
        } else if self_category != FpWheelCategory::Normal {
            return true;
        }
        self.0 == other.0
    }

    pub fn roughly_eq(&self, other: Self) -> bool {
        let self_category = self.0.get_category();
        let other_category = other.0.get_category();
        if self_category != other_category {
            return false;
        } else if self_category != FpWheelCategory::Normal {
            return true;
        }
        (self.0 - other.0) < 0.0001 && (self.0 - other.0) > -0.0001
    }

    fn add(&self, other: Self) -> Self {
        match (self.0.get_category(), other.0.get_category()) {
            (FpWheelCategory::Bottom, _) => Self::BOTTOM,
            (_, FpWheelCategory::Bottom) => Self::BOTTOM,
            (FpWheelCategory::Infinity, FpWheelCategory::Infinity) => Self::BOTTOM,
            (FpWheelCategory::Infinity, _) => Self::INFINITY,
            (_, FpWheelCategory::Infinity) => Self::INFINITY,
            (_, FpWheelCategory::Zero) => *self,
            (FpWheelCategory::Zero, _) => other,
            (FpWheelCategory::Normal, FpWheelCategory::Normal) => Wheel32(self.0 + other.0),
        }
    }

    fn mul(&self, other: Self) -> Self {
        match (self.0.get_category(), other.0.get_category()) {
            (FpWheelCategory::Bottom, _) => Self::BOTTOM,
            (_, FpWheelCategory::Bottom) => Self::BOTTOM,
            (FpWheelCategory::Infinity, FpWheelCategory::Zero) => Self::BOTTOM,
            (FpWheelCategory::Zero, FpWheelCategory::Infinity) => Self::BOTTOM,
            (_, FpWheelCategory::Infinity) => Self::INFINITY,
            (FpWheelCategory::Infinity, _) => Self::INFINITY,
            (FpWheelCategory::Zero, _) => Self::ZERO,
            (_, FpWheelCategory::Zero) => Self::ZERO,
            (FpWheelCategory::Normal, FpWheelCategory::Normal) => Wheel32(self.0 * other.0),
        }
    }

    fn neg(&self) -> Self {
       self.mul(Self::NEGATIVE_ONE)
    }

    pub fn inv(&self) -> Self {
        match self.0.get_category() {
            FpWheelCategory::Bottom => Self::BOTTOM,
            FpWheelCategory::Infinity => Self::ZERO,
            FpWheelCategory::Zero => Self::INFINITY,
            FpWheelCategory::Normal => Wheel32(1.0 / self.0),
        }
    }
}

impl Wheel for Wheel32 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    const INFINITY: Self = Self::INFINITY;
    const BOTTOM: Self = Self::BOTTOM;

    fn add(&self, other: &Self) -> Self {
        self.add(*other)
    }

    fn neg(&self) -> Self {
        self.neg()
    }

    fn mul(&self, other: &Self) -> Self {
        self.mul(*other)
    }

    fn inv(&self) -> Self {
        self.inv()
    }
}

impl PartialEq for Wheel32 {
    fn eq(&self, other: &Self) -> bool {
        self.eq(*other)
    }
}

impl Eq for Wheel32 {}

impl Debug for Wheel32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.get_category() {
            FpWheelCategory::Zero => write!(f, "Wheel32::ZERO"),
            FpWheelCategory::Infinity => write!(f, "Wheel32::INFINITY"),
            FpWheelCategory::Bottom => write!(f, "Wheel32::BOTTOM"),
            FpWheelCategory::Normal => write!(f, "Wheel32({})", self.0),
        }
    }
}

impl Display for Wheel32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.get_category() {
            FpWheelCategory::Zero => write!(f, "0"),
            FpWheelCategory::Infinity => write!(f, "Inf"),
            FpWheelCategory::Bottom => write!(f, "Bottom"),
            FpWheelCategory::Normal => write!(f, "{}", self.0),
        }
    }
}


// Conversion from floating point real numbers

impl From<f32> for Wheel32 {
    fn from(value: f32) -> Self {
        Wheel32(value)
    }
}


// Arithmetic operations

// Add

impl Add for Wheel32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(&self, other)
    }
}

impl Add<&Wheel32> for Wheel32 {
    type Output = Wheel32;

    fn add(self, other: &Wheel32) -> Wheel32 {
        self.add(*other)
    }
}

impl Add<Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn add(self, other: Wheel32) -> Wheel32 {
        (*self).add(other)
    }
}

impl Add<&Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn add(self, other: &Wheel32) -> Wheel32 {
        (*self).add(*other)
    }
}

// Sub

impl Sub for Wheel32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }
}

impl Sub<&Wheel32> for Wheel32 {
    type Output = Wheel32;

    fn sub(self, other: &Wheel32) -> Wheel32 {
        self.add(other.neg())
    }
}

impl Sub<Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn sub(self, other: Wheel32) -> Wheel32 {
        self.add(other.neg())
    }
}

impl Sub<&Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn sub(self, other: &Wheel32) -> Wheel32 {
        self.add(other.neg())
    }
}

// Mul

impl Mul for Wheel32 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::mul(&self, other)
    }
}

impl Mul<&Wheel32> for Wheel32 {
    type Output = Wheel32;

    fn mul(self, other: &Wheel32) -> Wheel32 {
        self.mul(*other)
    }
}

impl Mul<Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn mul(self, other: Wheel32) -> Wheel32 {
        (*self).mul(other)
    }
}

impl Mul<&Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn mul(self, other: &Wheel32) -> Wheel32 {
        (*self).mul(*other)
    }
}

// Div

impl Div for Wheel32 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.mul(other.inv())
    }
}

impl Div<&Wheel32> for Wheel32 {
    type Output = Wheel32;

    fn div(self, other: &Wheel32) -> Wheel32 {
        self.mul(other.inv())
    }
}

impl Div<Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn div(self, other: Wheel32) -> Wheel32 {
        (*self).mul(other.inv())
    }
}

impl Div<&Wheel32> for &Wheel32 {
    type Output = Wheel32;

    fn div(self, other: &Wheel32) -> Wheel32 {
        (*self).mul(other.inv())
    }
}

// Neg

impl Neg for Wheel32 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(&self)
    }
}

impl Neg for &Wheel32 {
    type Output = Wheel32;

    fn neg(self) -> Wheel32 {
        self.neg()
    }
}


// Implementations for Wheel64

impl Wheel64 {
    pub const ZERO: Self = Wheel64(0.0);
    pub const ONE: Self = Wheel64(1.0);
    pub const NEGATIVE_ONE: Self = Wheel64(-1.0);
    pub const INFINITY: Self = Wheel64(f64::INFINITY);
    pub const BOTTOM: Self = Wheel64(f64::NAN);

    pub fn new(value: f64) -> Self {
        Wheel64(value)
    }

    fn eq(&self, other: Self) -> bool {
        let self_category = self.0.get_category();
        let other_category = other.0.get_category();
        if self_category != other_category {
            return false;
        } else if self_category != FpWheelCategory::Normal {
            return true;
        }
        self.0 == other.0
    }

    pub fn roughly_eq(&self, other: Self) -> bool {
        let self_category = self.0.get_category();
        let other_category = other.0.get_category();
        if self_category != other_category {
            return false;
        } else if self_category != FpWheelCategory::Normal {
            return true;
        }
        (self.0 - other.0) < 0.0000001 && (self.0 - other.0) > -0.0000001
    }

    fn add(&self, other: Self) -> Self {
        match (self.0.get_category(), other.0.get_category()) {
            (FpWheelCategory::Bottom, _) => Self::BOTTOM,
            (_, FpWheelCategory::Bottom) => Self::BOTTOM,
            (FpWheelCategory::Infinity, FpWheelCategory::Infinity) => Self::BOTTOM,
            (FpWheelCategory::Infinity, _) => Self::INFINITY,
            (_, FpWheelCategory::Infinity) => Self::INFINITY,
            (_, FpWheelCategory::Zero) => *self,
            (FpWheelCategory::Zero, _) => other,
            (FpWheelCategory::Normal, FpWheelCategory::Normal) => Wheel64(self.0 + other.0),
        }
    }

    fn mul(&self, other: Self) -> Self {
        match (self.0.get_category(), other.0.get_category()) {
            (FpWheelCategory::Bottom, _) => Self::BOTTOM,
            (_, FpWheelCategory::Bottom) => Self::BOTTOM,
            (FpWheelCategory::Infinity, FpWheelCategory::Zero) => Self::BOTTOM,
            (FpWheelCategory::Zero, FpWheelCategory::Infinity) => Self::BOTTOM,
            (_, FpWheelCategory::Infinity) => Self::INFINITY,
            (FpWheelCategory::Infinity, _) => Self::INFINITY,
            (FpWheelCategory::Zero, _) => Self::ZERO,
            (_, FpWheelCategory::Zero) => Self::ZERO,
            (FpWheelCategory::Normal, FpWheelCategory::Normal) => Wheel64(self.0 * other.0),
        }
    }

    fn neg(&self) -> Self {
       self.mul(Self::NEGATIVE_ONE)
    }

    pub fn inv(&self) -> Self {
        match self.0.get_category() {
            FpWheelCategory::Bottom => Self::BOTTOM,
            FpWheelCategory::Infinity => Self::ZERO,
            FpWheelCategory::Zero => Self::INFINITY,
            FpWheelCategory::Normal => Wheel64(1.0 / self.0),
        }
    }
}

impl Wheel for Wheel64 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    const INFINITY: Self = Self::INFINITY;
    const BOTTOM: Self = Self::BOTTOM;

    fn add(&self, other: &Self) -> Self {
        self.add(*other)
    }

    fn neg(&self) -> Self {
        self.neg()
    }

    fn mul(&self, other: &Self) -> Self {
        self.mul(*other)
    }

    fn inv(&self) -> Self {
        self.inv()
    }
}

impl PartialEq for Wheel64 {
    fn eq(&self, other: &Self) -> bool {
        self.eq(*other)
    }
}

impl Eq for Wheel64 {}

impl Debug for Wheel64 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.get_category() {
            FpWheelCategory::Zero => write!(f, "Wheel64::ZERO"),
            FpWheelCategory::Infinity => write!(f, "Wheel64::INFINITY"),
            FpWheelCategory::Bottom => write!(f, "Wheel64::BOTTOM"),
            FpWheelCategory::Normal => write!(f, "Wheel64({})", self.0),
        }
    }
}

impl Display for Wheel64 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.get_category() {
            FpWheelCategory::Zero => write!(f, "0"),
            FpWheelCategory::Infinity => write!(f, "Inf"),
            FpWheelCategory::Bottom => write!(f, "Bottom"),
            FpWheelCategory::Normal => write!(f, "{}", self.0),
        }
    }
}


// Conversion from floating point real numbers

impl From<f64> for Wheel64 {
    fn from(value: f64) -> Self {
        Wheel64(value)
    }
}


// Arithmetic operations

// Add

impl Add for Wheel64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(&self, other)
    }
}

impl Add<&Wheel64> for Wheel64 {
    type Output = Wheel64;

    fn add(self, other: &Wheel64) -> Wheel64 {
        self.add(*other)
    }
}

impl Add<Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn add(self, other: Wheel64) -> Wheel64 {
        (*self).add(other)
    }
}

impl Add<&Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn add(self, other: &Wheel64) -> Wheel64 {
        (*self).add(*other)
    }
}

// Sub

impl Sub for Wheel64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }
}

impl Sub<&Wheel64> for Wheel64 {
    type Output = Wheel64;

    fn sub(self, other: &Wheel64) -> Wheel64 {
        self.add(other.neg())
    }
}

impl Sub<Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn sub(self, other: Wheel64) -> Wheel64 {
        self.add(other.neg())
    }
}

impl Sub<&Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn sub(self, other: &Wheel64) -> Wheel64 {
        self.add(other.neg())
    }
}

// Mul

impl Mul for Wheel64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::mul(&self, other)
    }
}

impl Mul<&Wheel64> for Wheel64 {
    type Output = Wheel64;

    fn mul(self, other: &Wheel64) -> Wheel64 {
        self.mul(*other)
    }
}

impl Mul<Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn mul(self, other: Wheel64) -> Wheel64 {
        (*self).mul(other)
    }
}

impl Mul<&Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn mul(self, other: &Wheel64) -> Wheel64 {
        (*self).mul(*other)
    }
}

// Div

impl Div for Wheel64 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.mul(other.inv())
    }
}

impl Div<&Wheel64> for Wheel64 {
    type Output = Wheel64;

    fn div(self, other: &Wheel64) -> Wheel64 {
        self.mul(other.inv())
    }
}

impl Div<Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn div(self, other: Wheel64) -> Wheel64 {
        (*self).mul(other.inv())
    }
}

impl Div<&Wheel64> for &Wheel64 {
    type Output = Wheel64;

    fn div(self, other: &Wheel64) -> Wheel64 {
        (*self).mul(other.inv())
    }
}

// Neg

impl Neg for Wheel64 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(&self)
    }
}

impl Neg for &Wheel64 {
    type Output = Wheel64;

    fn neg(self) -> Wheel64 {
        self.neg()
    }
}



#[cfg(test)]
mod test {
    use super::*;
    type MyWheel = w64;

    const ZERO: MyWheel = MyWheel::ZERO;
    const ONE: MyWheel = MyWheel::ONE;
    const INFINITY: MyWheel = MyWheel::INFINITY;
    const BOTTOM: MyWheel = MyWheel::BOTTOM;

    fn assert_eq(a: MyWheel, b: MyWheel) {
        assert!(a.roughly_eq(b));
    }

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
    fn half() -> MyWheel {
        MyWheel::new(0.5)
    }

    #[inline]
    fn negative_quarter() -> MyWheel {
        MyWheel::new(-0.25)
    }

    #[inline]
    fn any_numbers() -> [MyWheel; 9] {
        [
            ZERO, ONE, INFINITY, BOTTOM,
            negative_one(), three(), negative_two(),
            half(), negative_quarter()
        ]
    }

    #[test]
    fn inv_is_involution() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", x.inv().inv(), x);
            assert_eq(x.inv().inv(), x);
        }
    }

    #[test]
    fn inv_is_multicative() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                println!("{:?} == {:?}", (x * y).inv(), y.inv() * x.inv());
                assert_eq((x * y).inv(), y.inv() * x.inv());
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
                    assert_eq((x + y) * z + ZERO * z, x * z + y * z);
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
                    assert_eq((x + y * z) / y, x / y + z + ZERO * y);
                }
            }
        }
    }

    /// `0 * 0 = 0`
    #[test]
    fn zero_times_zero() {
        assert_eq(ZERO * ZERO, ZERO);
    }

    /// `(x + 0 * y) * z = x * z + 0 * y`
    #[test]
    fn zero_times_y() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                for &z in any_numbers().iter() {
                    println!("{:?} == {:?}", (x + ZERO * y) * z, x * z + ZERO * y);
                    assert_eq((x + ZERO * y) * z, x * z + ZERO * y);
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
                assert_eq((x + ZERO * y).inv(), x.inv() + ZERO * y);
            }
        }
    }

    /// `0 / 0 + x = 0 / 0`
    #[test]
    fn bottom_addition() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", BOTTOM + x, BOTTOM);
            assert_eq(BOTTOM + x, BOTTOM);
        }
    }

    /// `0 * x + 0 * y = 0 * x * y`
    #[test]
    fn zero_times_x_plus_zero_times_y() {
        for &x in any_numbers().iter() {
            for &y in any_numbers().iter() {
                println!("{:?} == {:?}", ZERO * x + ZERO * y, ZERO * x * y);
                assert_eq(ZERO * x + ZERO * y, ZERO * x * y);
            }
        }
    }

    /// `x / x = 1 + 0 * x / x`
    #[test]
    fn x_div_x() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", x / x, ONE + ZERO * x / x);
            assert_eq(x / x, ONE + ZERO * x / x);
        }
    }

    /// `x - x = 0 * x * x`
    #[test]
    fn x_minus_x() {
        for &x in any_numbers().iter() {
            println!("{:?} == {:?}", x - x, ZERO * x * x);
            assert_eq(x - x, ZERO * x * x);
        }
    }
}
