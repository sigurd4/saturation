use core::{f32, marker::PhantomData};

use num::Float;
use real_time_fir_iir_filters::f;

moddef::moddef!(
    flat(pub) mod {
        model
    }
);

pub struct DiodeClipper<F, M1, M2 = M1>
where
    F: Float,
    M1: DiodeModel,
    M2: DiodeModel
{
    r_d: F,
    marker: PhantomData<(M1, M2)>
}

impl<F, M1, M2> DiodeClipper<F, M1, M2>
where
    M1: DiodeModel,
    M2: DiodeModel,
    F: Float
{
    const I_0: [f64; 2] = [M1::I_0, M2::I_0];
    const ALPHA: [f64; 2] = [M1::Q_E/M1::ETA/M1::K/M1::T, M2::Q_E/M2::ETA/M2::K/M2::T];

    pub fn new(r_d: F) -> Self
    {
        Self {
            r_d,
            marker: PhantomData
        }
    }

    pub fn saturate(&self, x: F) -> F
    {
        let b = x.is_sign_negative();
        let vf = f!(Self::I_0[b as usize])*self.r_d;
        let alpha = f!(Self::ALPHA[b as usize]);
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

    use crate::{diodes::Diode1N4148, tubes::{Tube12AU7, Tube6550, Tube6DJ8, Tube6L6CG, TubeKT88}};

    use super::*;

    #[test]
    fn it_works()
    {
        const RANGE: Range<f32> = -10.0..10.0;
        
        let r_d = 1e3;
        
        let mut t0 = DiodeClipper::<_, Diode1N4148>::new(r_d);

        crate::tests::plot(
            "DiodeClipper",
            RANGE,
            |x| [
                t0.saturate(x)
            ]
        )
    }
}