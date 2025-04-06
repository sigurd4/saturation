use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use num::{Float, traits::FloatConst};

use crate::{Saturation, SaturationMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct SoftExp;

impl<F> SaturationMut<F, Range<F>> for SoftExp
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: Range<F>) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, Range<F>> for SoftExp
where
    F: Float + FloatConst
{
    #[inline]
    fn saturate(&self, x: F, range: Range<F>) -> F
    {
        if x.is_sign_negative()
        {
            self.saturate(x, range.start..)
        }
        else
        {
            self.saturate(x, ..range.end)
        }
    }
}
impl<F> SaturationMut<F, RangeFrom<F>> for SoftExp
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: RangeFrom<F>) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, RangeFrom<F>> for SoftExp
where
    F: Float + FloatConst
{
    #[inline]
    fn saturate(&self, mut x: F, range: RangeFrom<F>) -> F
    {
        assert!(range.start <= F::zero(), "Lower bound must be negative");
        x = x.max(range.start);
        x + (range.start - x).exp() - range.start.exp()
    }
}
impl<F> SaturationMut<F, RangeTo<F>> for SoftExp
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: RangeTo<F>) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, RangeTo<F>> for SoftExp
where
    F: Float + FloatConst
{
    #[inline]
    fn saturate(&self, mut x: F, range: RangeTo<F>) -> F
    {
        assert!(range.end >= F::zero(), "Upper bound must be positive");
        x = x.min(range.end);
        x - (x - range.end).exp() + (-range.end).exp()
    }
}

impl<F> SaturationMut<F, RangeInclusive<F>> for SoftExp
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: RangeInclusive<F>) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, RangeInclusive<F>> for SoftExp
where
    F: Float + FloatConst
{
    #[inline]
    fn saturate(&self, x: F, range: RangeInclusive<F>) -> F
    {
        self.saturate(x, *range.start()..*range.end())
    }
}
impl<F> SaturationMut<F, RangeToInclusive<F>> for SoftExp
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: RangeToInclusive<F>) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, RangeToInclusive<F>> for SoftExp
where
    F: Float + FloatConst
{
    #[inline]
    fn saturate(&self, x: F, range: RangeToInclusive<F>) -> F
    {
        self.saturate(x, ..range.end)
    }
}
impl<F> SaturationMut<F, RangeFull> for SoftExp
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: RangeFull) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, RangeFull> for SoftExp
where
    F: Float + FloatConst
{
    #[inline]
    fn saturate(&self, x: F, RangeFull: RangeFull) -> F
    {
        x
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn it_works()
    {
        let range = -2.0..2.0;
        let clip = -1.2..1.0;

        crate::tests::plot(
            "SoftExp",
            range,
            |x| [
                SoftExp.saturate(x, clip.clone().start..),
                SoftExp.saturate(x, clip.clone()),
                SoftExp.saturate(x, ..clip.clone().end),
            ]
        )
    }
}
