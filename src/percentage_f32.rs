use std::ops::{
    Add,
    Sub,
};

#[derive(Clone, Copy, Debug)]
pub struct PercentageF32(f32);

impl PercentageF32 {
    #[inline]
    pub fn try_new(with: f32) -> Option<Self> {
        if 0f32 <= with && with <= 1f32 {
            Some(Self(with))
        } else {
            None
        }
    }

    #[inline]
    pub fn new_or_panic(with: f32) -> Self {
        Self::try_new(with).expect(&format!(
            "Attempted to create new PercentageF32 using invalid value ({})",
            with
        ))
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Add for PercentageF32 {
    type Output = Option<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::try_new(self.0 + rhs.0)
    }
}

impl Sub for PercentageF32 {
    type Output = Option<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::try_new(self.0 - rhs.0)
    }
}

impl TryFrom<f32> for PercentageF32 {
    type Error = f32;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_new(value).ok_or(value)
    }
}

impl From<PercentageF32> for f32 {
    #[inline]
    fn from(value: PercentageF32) -> Self {
        value.value()
    }
}
