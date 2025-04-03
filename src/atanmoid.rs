use core::ops::RangeFull;

use num::{traits::FloatConst, Float};

use crate::Saturation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ATanMoid;

impl<F> Saturation<F, RangeFull> for ATanMoid
where
    F: Float + FloatConst
{
    fn saturate(&self, x: F, RangeFull: RangeFull) -> F
    {
        let frac_2_pi = F::FRAC_2_PI();
        frac_2_pi*(x/frac_2_pi).atan()
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
            "ATanMoid",
            range,
            |x| [
                ATanMoid.saturate(x, ..),
            ]
        )
    }
}