use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};

#[derive(Clone, Copy, Debug)]
pub struct CircleDegrees(i16);

impl CircleDegrees {
    #[inline]
    pub const fn new_exact(with: i16) -> Option<Self> {
        match with {
            0..=359 => Some(Self(with)),
            _ => None,
        }
    }

    #[inline]
    pub const fn new_wrapped(with: i16) -> Self {
        Self(with % 360)
    }

    #[inline]
    pub fn value(&self) -> i16 {
        self.0
    }
}

impl Add for CircleDegrees {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new_wrapped(self.0 + rhs.0)
    }
}

impl AddAssign for CircleDegrees {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub for CircleDegrees {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_wrapped(self.0 - rhs.0)
    }
}

impl SubAssign for CircleDegrees {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<CircleDegrees> for i16 {
    #[inline]
    fn from(value: CircleDegrees) -> Self {
        value.value()
    }
}
