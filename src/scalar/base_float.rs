pub trait BaseFloat: Copy {
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn recip(self) -> Self;
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct float<T: BaseFloat + Clone + Copy>(pub T);

impl BaseFloat for core::primitive::f32 {
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
}

impl BaseFloat for core::primitive::f64 {
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
}

impl<T: BaseFloat> core::ops::Add for float<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        float(BaseFloat::add(self.0, rhs.0))
    }
}

impl<T: BaseFloat> core::ops::Sub for float<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        float(BaseFloat::sub(self.0, rhs.0))
    }
}

impl<T: BaseFloat> core::ops::Mul for float<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        float(BaseFloat::mul(self.0, rhs.0))
    }
}

impl<T: BaseFloat> core::ops::Div for float<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        float(BaseFloat::div(self.0, rhs.0))
    }
}

impl<T: BaseFloat> BaseFloat for float<T> {
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
        Self(BaseFloat::recip(self.0))
    }
}

impl<T: BaseFloat + Clone + Copy> From<T> for float<T> {
    #[inline]
    fn from(value: T) -> Self {
        float(value)
    }
}
