use core::ops::RangeFull;

use num::Float;

use crate::{Saturate, SaturateMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct PythMoid;

impl<F> SaturateMut<F, RangeFull> for PythMoid
where
    F: Float
{
    fn saturate_mut(&mut self, x: F, range: RangeFull) -> F
    {
        self.saturate(x, range)
    }
}
impl<F> Saturate<F, RangeFull> for PythMoid
where
    F: Float
{
    fn saturate(&self, x: F, RangeFull: RangeFull) -> F
    {
        x/(F::one() + x*x).sqrt()
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
            "PythMoid",
            range,
            |x| [
                PythMoid.saturate(x, ..),
            ]
        )
    }
}