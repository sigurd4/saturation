use real_time_fir_iir_filters::param::FilterFloat;

use super::{TriodeClassA, TriodeModel};

use crate::f;

pub trait TriodeCalc<F, M>
where
    F: FilterFloat,
    M: TriodeModel
{
    fn reset(&mut self)
    {
        
    }
    fn param(&self) -> &TriodeClassA<F>;
    fn param_mut(&mut self) -> &mut TriodeClassA<F>;
    fn vp_a(&mut self, vg: F) -> [F; 2];
}
impl<F, M> TriodeCalc<F, M> for TriodeClassA<F>
where
    F: FilterFloat,
    M: TriodeModel
{
    fn param(&self) -> &TriodeClassA<F>
    {
        self
    }
    fn param_mut(&mut self) -> &mut TriodeClassA<F>
    {
        self
    }
    fn vp_a(&mut self, vg: F) -> [F; 2]
    {
        let TriodeClassA {r_i: _, r_p: rp, v_pp: vpp, v_c: _} = *self;
        let two_rp = rp + rp;
        let one = F::one();
        let zero = F::zero();

        let mu_inv = f!(1.0/M::MU);
        let mu = f!(M::MU);
        let kp = f!(M::K_P);
        let kvb = f!(M::K_VB);
        let kg1 = f!(M::K_G1);
        let ex = f!(M::EX);

        let v1_max = (vpp*kg1/two_rp).powf(ex.recip());
        
        //let mut v1 = vp/kp*(kp*(mu_inv + vg/(kvb + vp*vp).sqrt())).exp().ln_1p();
        //let mut vp = vpp - two_rp*v1.max(zero).powf(ex)/kg1;

        //let mut vp = vpp - two_rp*(vp/kp*(kp*(mu_inv + vg/(kvb + vp*vp).sqrt())).exp().ln_1p()).max(zero).powf(ex)/kg1;

        /*let mut vp = {
            let v1 = crate::exp_ln_1p(vpp + vg*mu)/(mu + two_rp/kg1);
            vpp - two_rp*v1.max(zero).powf(ex)/kg1
        };*/

        //let vp = (vpp - vg*rp/kg1)/(one + rp/mu/kg1);
        //let mut v1 = vp/kp*(kp*(mu_inv + vg/(kvb + vp*vp).sqrt())).exp().ln_1p();
        
        let mut v1 = {
            let mut vp = vpp/(one + rp/(mu*kg1));
            vp = {
                let b = crate::exp_ln_1p(kp*(mu_inv + vg/(kvb + vp*vp).sqrt()));
                let v1 = vpp/(kp/b + two_rp/kg1);
                vpp - two_rp*v1/kg1
            };
            let b = crate::exp_ln_1p(kp*(mu_inv + vg/(kvb + vp*vp).sqrt()));
            vpp/(kp/b + two_rp/kg1)
        };

        const NEWTON: usize = 2;

        for _ in 0..NEWTON
        {
            v1 = v1.max(zero).min(v1_max);
            let vp = (vpp - two_rp/kg1*v1.powf(ex)).max(zero);
            let dvp_dv1 = -two_rp/kg1*ex*v1.powf(ex - one);

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

        v1 = v1.max(zero).min(v1_max);
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

        [vp, -dvp_dvg]
    }
}