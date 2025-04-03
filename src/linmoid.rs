use core::ops::RangeFull;

use num::Float;

use crate::Saturation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct LinMoid;

impl<F> Saturation<F, RangeFull> for LinMoid
where
    F: Float
{
    fn saturate(&self, x: F, RangeFull: RangeFull) -> F
    {
        x/(F::one() + x.abs())
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
            "LinMoid",
            range,
            |x| [
                LinMoid.saturate(x, ..),
            ]
        )
    }
}