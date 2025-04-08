use real_time_fir_iir_filters::{conf::LowPass, filters::iir::first::FirstOrderRCFilter, param::{FilterFloat, RC}, static_rtf::StaticRtfBase};

use super::{PentodeClassA, PentodeModel};

use crate::{f, rtf::Rtf1};

pub trait PentodeCathodeFilter<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    type Param;

    fn new_cathode_filter(param: Self::Param) -> Self;

    fn param_cathode(&self) -> &Self::Param;
    fn param_cathode_mut(&mut self) -> &mut Self::Param;

    fn vg_cathode(&mut self, param: PentodeClassA<F>, miller_effect: F, rate: F, x: F) -> F;
}
impl<F, M> PentodeCathodeFilter<F, M> for ()
where
    F: FilterFloat,
    M: PentodeModel
{
    type Param = ();

    fn new_cathode_filter(_: Self::Param) -> Self
    {
        
    }

    fn param_cathode(&self) -> &Self::Param
    {
        self
    }
    fn param_cathode_mut(&mut self) -> &mut Self::Param
    {
        self
    }

    fn vg_cathode(&mut self, _: PentodeClassA<F>, _: F, _: F, x: F) -> F
    {
        x
    }
}
impl<F, M> PentodeCathodeFilter<F, M> for FirstOrderRCFilter<LowPass, F, RC<F>>
where
    F: FilterFloat,
    M: PentodeModel,
    Self: Rtf1<F = F>
{
    type Param = RC<F>;

    fn new_cathode_filter(param: Self::Param) -> Self
    {
        Self::new::<LowPass>(param)
    }

    fn param_cathode(&self) -> &Self::Param
    {
        self.get_param()
    }
    fn param_cathode_mut(&mut self) -> &mut Self::Param
    {
        self.get_param_mut()
    }

    fn vg_cathode(&mut self, param: PentodeClassA<F>, miller_effect: F, rate: F, x: F) -> F
    {
        x - self.filter(rate, (x*miller_effect - x)*self.param.r/param.r_p)
    }
}

pub trait PentodeFilter<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    fn new_input_filter(r_i: F) -> Self;
    fn new_output_filter(r_p: F) -> Self;

    fn update_miller_effect_input(&mut self, miller_effect: F);
    fn update_miller_effect_output(&mut self, miller_effect: F);

    fn vg(&mut self, param: PentodeClassA<F>, rate: F, x: F) -> F;
    fn y(&mut self, rate: F, y: F) -> F;
}
impl<F, M> PentodeFilter<F, M> for ()
where
    F: FilterFloat,
    M: PentodeModel
{
    fn new_input_filter(_: F) -> Self
    {
        
    }
    fn new_output_filter(_: F) -> Self
    {
        
    }

    fn update_miller_effect_input(&mut self, _: F)
    {

    }
    fn update_miller_effect_output(&mut self, _: F)
    {

    }

    fn vg(&mut self, param: PentodeClassA<F>, _: F, x: F) -> F
    {
        let ri = param.r_i;
        let rgi = f!(M::R_GI);

        x*rgi/(rgi + ri) - param.v_c
    }
    fn y(&mut self, _: F, y: F) -> F
    {
        y
    }
}
impl<F, M> PentodeFilter<F, M> for FirstOrderRCFilter<LowPass, F, RC<F>>
where
    F: FilterFloat,
    M: PentodeModel,
    Self: Rtf1<F = F>
{
    fn new_input_filter(r_i: F) -> Self
    {
        FirstOrderRCFilter::new::<LowPass>(RC {r: r_i, c: f!(M::C_CG + M::C_PG)})
    }
    fn new_output_filter(r_p: F) -> Self
    {
        FirstOrderRCFilter::new::<LowPass>(RC {r: r_p, c: f!(M::C_CP + M::C_PG)})
    }

    fn update_miller_effect_input(&mut self, miller_effect: F)
    {
        self.param.c = f!(M::C_CG) + f!(M::C_PG)*miller_effect;
    }
    fn update_miller_effect_output(&mut self, miller_effect: F)
    {
        self.param.c = f!(M::C_CP) + f!(M::C_PG)*miller_effect;
    }

    fn vg(&mut self, param: PentodeClassA<F>, rate: F, x: F) -> F
    {
        self.filter(rate, PentodeFilter::<F, M>::vg(&mut (), param, rate, x))
    }
    fn y(&mut self, rate: F, y: F) -> F
    {
        self.filter(rate, PentodeFilter::<F, M>::y(&mut (), rate, y))
    }
}