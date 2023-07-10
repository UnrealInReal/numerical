use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub trait BaseFloat:
    Copy
    + Clone
    + PartialEq
    + PartialOrd
    + From<f32>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + core::fmt::Debug
{
    const TWO: Self;
    const ONE: Self;
    const ZERO: Self;

    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn recip(self) -> Self;
    fn epsilon() -> Self;
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn signum(self) -> Self;

    #[inline]
    fn half(self) -> Self {
        self * Self::TWO.recip()
    }

    #[inline]
    fn double(self) -> Self {
        self * Self::TWO
    }
}

impl BaseFloat for core::primitive::f32 {
    const ZERO: Self = 0_f32;
    const ONE: Self = 1_f32;
    const TWO: Self = 2_f32;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        core::ops::Add::add(self, rhs)
    }
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        core::ops::Sub::sub(self, rhs)
    }
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        core::ops::Mul::mul(self, rhs)
    }
    #[inline]
    fn div(self, rhs: Self) -> Self {
        core::ops::Div::div(self, rhs)
    }
    #[inline]
    fn recip(self) -> Self {
        Self::recip(self)
    }
    #[inline]
    fn epsilon() -> Self {
        Self::EPSILON
    }
    #[inline]
    fn abs(self) -> Self {
        f32::abs(self)
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
    #[inline]
    fn signum(self) -> Self {
        f32::signum(self)
    }
}

impl BaseFloat for core::primitive::f64 {
    const ZERO: Self = 0_f64;
    const ONE: Self = 1_f64;
    const TWO: Self = 2_f64;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        core::ops::Add::add(self, rhs)
    }
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        core::ops::Sub::sub(self, rhs)
    }
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        core::ops::Mul::mul(self, rhs)
    }
    #[inline]
    fn div(self, rhs: Self) -> Self {
        core::ops::Div::div(self, rhs)
    }
    #[inline]
    fn recip(self) -> Self {
        Self::recip(self)
    }
    #[inline]
    fn epsilon() -> Self {
        Self::EPSILON
    }
    #[inline]
    fn abs(self) -> Self {
        f64::abs(self)
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
    #[inline]
    fn signum(self) -> Self {
        f64::signum(self)
    }
}
