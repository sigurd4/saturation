use core::marker::PhantomData;

use num::Float;
use crate::f;

moddef::moddef!(
    flat(pub) mod {
        model
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DiodeClipper<F, M>
where
    F: Float,
    M: DiodeModel
{
    r_d: F,
    marker: PhantomData<M>
}

impl<F, M> DiodeClipper<F, M>
where
    M: DiodeModel,
    F: Float
{
    /// Temperature (Kelvin)
    const T: f64 = 20.0 + 273.15;
    /// Electron charge
    const Q_E: f64 = 1.602176634e-19;
    /// Boltzmann constant
    const K: f64 = 1.38e-23;

    pub fn new(r_d: F) -> Self
    {
        Self {
            r_d,
            marker: PhantomData
        }
    }

    pub fn saturate(&self, x: F) -> F
    {
        let vf = f!(M::I_0)*self.r_d;
        let alpha = f!(Self::Q_E/M::ETA/Self::K/Self::T);
        let x_abs = x.abs();
        let e = (vf*alpha).ln() + (vf + x_abs)*alpha;
        let l = f!(crate::lambertw(e));
        x.signum()*(x_abs + vf - l/alpha)
    }
}

#[cfg(test)]
mod test
{
    use core::ops::Range;

    use crate::diodes::{Diode1N34A, Diode1N4001, Diode1N4148, Diode1N914};

    use super::*;

    #[test]
    fn it_works()
    {
        const RANGE: Range<f32> = -10.0..10.0;
        
        let r_d = 1e3;
        
        let t0 = DiodeClipper::<_, Diode1N4148>::new(r_d);
        let t1 = DiodeClipper::<_, Diode1N914>::new(r_d);
        let t2 = DiodeClipper::<_, Diode1N4001>::new(r_d);
        let t3 = DiodeClipper::<_, Diode1N34A>::new(r_d);

        crate::tests::plot(
            "DiodeClipper",
            RANGE,
            |x| [
                t0.saturate(x),
                t1.saturate(x),
                t2.saturate(x),
                t3.saturate(x)
            ]
        )
    }
}