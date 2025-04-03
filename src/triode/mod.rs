use core::marker::PhantomData;

use real_time_fir_iir_filters::{f, change::Change, conf::LowPass, filters::iir::first::FirstOrderRCFilter, param::{FilterFloat, RC}};

use crate::{rtf::Rtf1, tubes::Tube12AX7};

moddef::moddef!(
    flat(pub) mod {
        model,
        param
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Triode<F, M = Tube12AX7>
where
    F: FilterFloat,
    M: TriodeModel
{
    param: TriodeClassA<F>,
    input_filter: FirstOrderRCFilter<LowPass, F>,
    output_filter: FirstOrderRCFilter<LowPass, F>,
    marker: PhantomData<M>
}

impl<F, M> Triode<F, M>
where
    F: FilterFloat,
    M: TriodeModel
{
    pub fn new(param: TriodeClassA<F>) -> Self
    {
        let input_filter = FirstOrderRCFilter::new::<LowPass>(RC {r: param.r_i, c: f!(M::C_CG + M::C_PG)});
        let output_filter = FirstOrderRCFilter::new::<LowPass>(RC {r: param.r_p, c: f!(M::C_CP + M::C_PG)});
        Self {
            param,
            input_filter,
            output_filter,
            marker: PhantomData
        }
    }

    pub fn saturate(&mut self, rate: F, x: F) -> F
    where
        FirstOrderRCFilter<LowPass, F>: Rtf1<F = F>
    {
        // Math: https://www.normankoren.com/Audio/Tubemodspice_article.html

        let one = F::one();
        let zero = F::zero();

        let ri = self.param.r_i;
        let rp = self.param.r_p;
        let two_rp = rp + rp;
        let vpp = self.param.v_pp;
        let rgi = f!(M::R_GI);

        self.input_filter.param.r = (f!(1.0/M::R_GI) + ri.recip()).recip();

        let vg = self.input_filter.filter(rate, x*rgi/(rgi + ri));

        let (vp, a) = {
            let mu_inv = f!(1.0/M::MU);
            let mu = f!(M::MU);
            let kp = f!(M::K_P);
            let kvb = f!(M::K_VB);
            let kg1 = f!(M::K_G1);
            let ex = f!(M::EX);
            
            //let mut v1 = vp/kp*(kp*(mu_inv + vg/(kvb + vp*vp).sqrt())).exp().ln_1p();
            //let mut vp = vpp - two_rp*v1.max(zero).powf(ex)/kg1;

            let mut v1 = {
                let vp = vpp/(one + two_rp/mu/kg1);
                vp/kp*crate::exp_ln_1p(kp*(mu_inv + vg/(kvb + vp*vp).sqrt()))
            };

            const NEWTON: usize = 4;

            for _ in 0..NEWTON
            {
                v1 = v1.max(zero);
                let vp = (vpp - two_rp*v1.powf(ex)/kg1).max(zero);
                let dvp_dv1 = -two_rp*ex*v1.powf(ex - one)/kg1;

                let term = kvb + vp*vp;
                let term_sqrt = term.sqrt();

                let b = mu_inv + vg/term_sqrt;
                let db_dv1 = -vg*vp*dvp_dv1/(term*term_sqrt);

                let c = crate::exp_ln_1p(kp*b);
                let dc_dv1_d_kp = (kp*b - c).exp()*db_dv1;
                
                let f = v1 - vp*c/kp;
                let df_dv1 = one - dvp_dv1*c/kp - dc_dv1_d_kp*vp;

                let delta = f/df_dv1;
                v1 = v1 - delta;
            }

            v1 = v1.max(zero);
            let vp = (vpp - two_rp*v1.powf(ex)/kg1).max(zero);
            let dvp_dv1 = -two_rp*ex*v1.powf(ex - one)/kg1;

            let term = kvb + vp*vp;
            let term_sqrt = term.sqrt();

            let b = mu_inv + vg/term_sqrt;
            let db_dv1 = -vg*vp*dvp_dv1/(term*term_sqrt);

            let c = crate::exp_ln_1p(kp*b);
            let dc_dv1_d_kp = (kp*b - c).exp()*db_dv1;
            
            let df_dv1 = one - dvp_dv1*c/kp - dc_dv1_d_kp*vp;

            let df_dvg = vp/term_sqrt/(one + (-kp*b).exp());
            let dvp_dvg = df_dvg/df_dv1*dvp_dv1;

            (vp, dvp_dvg)
        };

        let miller_effect = one + a.max(zero);
        let change = crate::change(rate);

        self.output_filter.param.c.change(f!(M::C_CP) + f!(M::C_PG)*miller_effect, change);
        self.input_filter.param.c.change(f!(M::C_CG) + f!(M::C_PG)/miller_effect, change);

        self.output_filter.filter(rate, vp)
    }
}

#[cfg(test)]
mod test
{
    use core::ops::Range;

    use crate::tubes::{Tube12AU7, Tube6550, Tube6DJ8, Tube6L6CG, TubeKT88};

    use super::*;

    #[test]
    fn it_works()
    {
        const RATE: f32 = 100.0;
        const RANGE: Range<f32> = -2.0..2.0;
        
        let param = TriodeClassA {
            r_i: 1e3,
            r_p: 47e3,
            v_pp: 24.0
        };
        
        let mut t0 = Triode::<_, Tube6DJ8>::new(param);
        let mut t1 = Triode::<_, Tube12AX7>::new(param);
        let mut t2 = Triode::<_, Tube12AU7>::new(param);
        let mut t3 = Triode::<_, Tube6L6CG>::new(param);
        let mut t4 = Triode::<_, Tube6550>::new(param);
        let mut t5 = Triode::<_, TubeKT88>::new(param);

        crate::tests::plot(
            "Triode",
            RANGE,
            |x| [
                t0.saturate(RATE, x),
                t1.saturate(RATE, x),
                t2.saturate(RATE, x),
                t3.saturate(RATE, x),
                t4.saturate(RATE, x),
                t5.saturate(RATE, x)
            ]
        )
    }
}