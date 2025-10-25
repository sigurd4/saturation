use core::{marker::PhantomData, ops::RangeTo};

use num::Float;
use crate::{f, Saturate, SoftExp};

use super::JFETModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct JFETBuffer<F, M>
where
    F: Float,
    M: JFETModel
{
    r_s: F,
    v_dd: F,
    marker: PhantomData<M>
}

impl<F, M> JFETBuffer<F, M>
where
    F: Float,
    SoftExp: Saturate<F, RangeTo<F>>,
    M: JFETModel
{
    pub fn new(r_s: F, v_dd: F) -> Self
    {
        Self {
            r_s,
            v_dd,
            marker: PhantomData
        }
    }

    pub fn saturate(&self, x: F) -> F
    {
        const R_DS: f64 = 2.0;

        let zero = F::zero();
        let one = F::one();

        let vgo = x - f!(M::VTO);
        let two_beta = f!(M::BETA*2.0);
        let four_beta = f!(M::BETA*4.0);
        let mut vs = vgo + (one - (four_beta*self.r_s*vgo + one).sqrt())/(two_beta*self.r_s);

        let id = vs/self.r_s;
        vs = SoftExp.saturate(vs, ..(self.v_dd - id*f!(R_DS)).max(zero));

        vs = vs + f!(M::VTO) - (one - (-four_beta*self.r_s*f!(M::VTO) + one).sqrt())/(two_beta*self.r_s);
        vs
    }
}

#[cfg(test)]
mod test
{
    use core::ops::Range;

    use crate::jfets::JFET2N5458;

    use super::*;

    #[test]
    fn it_works()
    {
        const RANGE: Range<f32> = -5.0..20.0;
        
        let r_s = 100e3;
        
        let t0 = JFETBuffer::<_, JFET2N5458>::new(r_s, 9.0);

        crate::tests::plot(
            "JFETBuffer",
            RANGE,
            |x| [
                t0.saturate(x),
            ]
        )
    }
}