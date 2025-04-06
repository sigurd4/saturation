use real_time_fir_iir_filters::param::FilterFloat;

use super::{PentodeClassA, PentodeModel};

use crate::f;

pub trait PentodeCalc<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    fn reset(&mut self)
    {
        
    }
    fn param(&self) -> &PentodeClassA<F>;
    fn param_mut(&mut self) -> &mut PentodeClassA<F>;
    fn vp_a(&mut self, vg: F) -> [F; 2];
}
impl<F, M> PentodeCalc<F, M> for PentodeClassA<F>
where
    F: FilterFloat,
    M: PentodeModel
{
    fn param(&self) -> &PentodeClassA<F>
    {
        self
    }
    fn param_mut(&mut self) -> &mut PentodeClassA<F>
    {
        self
    }
    fn vp_a(&mut self, vg: F) -> [F; 2]
    {
        let PentodeClassA {r_i: _, r_p: rp, v_g2: vg2, v_pp: vpp, v_c: _} = *self;
        let two_rp = rp + rp;
        let one = F::one();
        let zero = F::zero();

        let mu_inv = f!(1.0/M::MU);
        let kp = f!(M::K_P);
        let kvb = f!(M::K_VB);
        let kg1 = f!(M::K_G1);
        let ex = f!(M::EX);

        let vg2_d_kp = vg2/kp;
        let c = kp*(mu_inv + vg/vg2);
        
        let v1 = vg2_d_kp*crate::exp_ln_1p(c);

        if v1.is_sign_positive()
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

            const NEWTON: usize = 2;

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

            [vp, dvp_dvg]
        }
        else
        {
            [vpp, zero]
        }
    }
}