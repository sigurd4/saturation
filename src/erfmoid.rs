use core::ops::RangeFull;

use num::{traits::FloatConst, Float};

use crate::{Saturation, SaturationMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ErfMoid;

impl<F> SaturationMut<F, RangeFull> for ErfMoid
where
    F: Float + FloatConst
{
    fn saturate_mut(&mut self, x: F, range: RangeFull) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturation<F, RangeFull> for ErfMoid
where
    F: Float + FloatConst
{
    fn saturate(&self, x: F, RangeFull: RangeFull) -> F
    {
        (x/F::FRAC_2_SQRT_PI()).erf()
    }
}

trait Erf: Float
{
    fn erf(self) -> Self;
}
impl<F> Erf for F
where
    F: Float
{
    default fn erf(self) -> Self
    {
        F::from(libm::erf(self.to_f64().unwrap())).unwrap()
    }
}
impl Erf for f64
{
    fn erf(self) -> Self
    {
        libm::erf(self)
    }
}
impl Erf for f32
{
    fn erf(self) -> Self
    {
        libm::erff(self)
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

        crate::tests::plot(
            "ErfMoid",
            range,
            |x| [
                ErfMoid.saturate(x, ..),
            ]
        )
    }
}