use core::ops::RangeFull;

use num::{traits::FloatConst, Float};

use crate::{ATanMoid, Saturation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct SinHATanMoid;

impl<F> Saturation<F, RangeFull> for SinHATanMoid
where
    F: Float + FloatConst
{
    fn saturate(&self, x: F, RangeFull: RangeFull) -> F
    {
        ATanMoid.saturate(x, ..).atan()
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
            "SinHATanMoid",
            range,
            |x| [
                SinHATanMoid.saturate(x, ..),
            ]
        )
    }
}