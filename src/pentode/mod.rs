use core::marker::PhantomData;

use real_time_fir_iir_filters::{conf::LowPass, change::Change, f, filters::iir::first::FirstOrderRCFilter, param::{FilterFloat, RC}};

use crate::{rtf::Rtf1, tubes::Tube6550};

moddef::moddef!(
    flat(pub) mod {
        model,
        param
    }
);

pub struct Pentode<F, M = Tube6550>
where
    F: FilterFloat,
    M: PentodeModel
{
    param: PentodeClassA<F>,
    input_filter: FirstOrderRCFilter<LowPass, F>,
    output_filter: FirstOrderRCFilter<LowPass, F>,
    marker: PhantomData<M>
}

impl<F, M> Pentode<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    pub fn new(param: PentodeClassA<F>) -> Self
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
        let vg2 = self.param.v_g2;
        let vpp = self.param.v_pp;
        let rgi = f!(M::R_GI);

        self.input_filter.param.r = (f!(1.0/M::R_GI) + ri.recip()).recip();

        let vg = self.input_filter.filter(rate, x*rgi/(rgi + ri));

        let mu_inv = f!(1.0/M::MU);
        let kp = f!(M::K_P);
        let kvb = f!(M::K_VB);
        let kg1 = f!(M::K_G1);
        let ex = f!(M::EX);

        let vg2_d_kp = vg2/kp;
        let c = kp*(mu_inv + vg/vg2);
        
        let v1 = vg2_d_kp*crate::exp_ln_1p(c);

        let (vp, a) = if v1.is_sign_positive()
        {
            let rp_inv = two_rp.recip();
            let vpp_d_rp = vpp/two_rp;

            let b = v1.powf(ex)/kg1;

            let mut vp = vpp/(one + b*two_rp/kvb);
            if vp > one
            {
                vp = vpp - b*two_rp*F::FRAC_PI_2();
            }
            vp = vp.max(zero).min(vpp);

            const NEWTON: usize = 10;

            for _ in 0..NEWTON
            {
                let vp_d_kvb = vp/kvb;

                let f = vpp_d_rp - vp/two_rp - b*vp.atan2(kvb);
                let df_dvp = -rp_inv - b/(vp*vp_d_kvb + kvb);

                let delta = f/df_dvp;
                vp = vp - delta;
            }

            vp = vp.max(zero).min(vpp);
            let vp_d_kvb = vp/kvb;

            let df_dvp = -rp_inv - b/(vp*vp_d_kvb + kvb);

            let dv1_dvg = (one + (-c).exp()).recip();
            let df_dv1 = -ex*v1.powf(ex - one)/kg1*vp.atan2(kvb);
            let dvp_dvg = dv1_dvg*df_dv1/df_dvp;

            (vp, dvp_dvg)
        }
        else
        {
            (vpp, zero)
        };

        let miller_effect = one + a.max(zero);
        let change = crate::change(rate);

        self.output_filter.param.c.change(f!(M::C_CP) + f!(M::C_PG)*miller_effect, change);
        self.input_filter.param.c.change(f!(M::C_CG) + f!(M::C_PG)/miller_effect, change);

        let vp = self.output_filter.filter(rate, vp);
        vp
    }
}

#[cfg(test)]
mod test
{
    use core::ops::Range;

    use crate::tubes::{Tube6550, Tube6L6CG, TubeKT88};

    use super::*;

    #[test]
    fn it_works()
    {
        const RATE: f32 = 100.0;
        const RANGE: Range<f32> = -2.0..50.0;
        
        let param = PentodeClassA {
            r_i: 1e3,
            r_p: 1e3,
            v_pp: 24.0,
            v_g2: 3.3
        };
        
        let mut t0 = Pentode::<_, Tube6L6CG>::new(param);
        let mut t1 = Pentode::<_, Tube6550>::new(param);
        let mut t2 = Pentode::<_, TubeKT88>::new(param);

        crate::tests::plot(
            "Pentode",
            RANGE,
            |x| [
                t0.saturate(RATE, x),
                t1.saturate(RATE, x),
                t2.saturate(RATE, x)
            ]
        )
    }
}