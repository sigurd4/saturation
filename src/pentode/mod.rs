use real_time_fir_iir_filters::{conf::LowPass, filters::iir::first::FirstOrderRCFilter, param::FilterFloat};

use crate::tubes::Tube6550;

moddef::moddef!(
    flat(pub) mod {
        cache,
        model,
        param
    },
    flat mod {
        calc,
        filter
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Pentode<F, M = Tube6550, FI = FirstOrderRCFilter<LowPass, F>, FO = FirstOrderRCFilter<LowPass, F>, C = PentodeCache<F, M>>
where
    F: FilterFloat,
    M: PentodeModel,
    C: PentodeCalc<F, M>,
    FI: PentodeFilter<F, M>,
    FO: PentodeFilter<F, M>
{
    calc: C,
    input_filter: FI,
    output_filter: FO,
    miller_effect: F,
    offset: F,
    model: M
}

impl<F, M, C, FI, FO> Pentode<F, M, FI, FO, C>
where
    F: FilterFloat,
    M: PentodeModel,
    C: PentodeCalc<F, M>,
    FI: PentodeFilter<F, M>,
    FO: PentodeFilter<F, M>
{
    pub fn new(calc: C, model: M) -> Self
    {
        let input_filter = FI::new_input_filter(calc.param().r_i);
        let output_filter = FO::new_output_filter(calc.param().r_p);
        let mut pentode = Self {
            calc,
            input_filter,
            output_filter,
            miller_effect: F::one(),
            offset: F::zero(),
            model,
        };
        pentode.calibrate();
        pentode
    }

    pub fn param(&self) -> &PentodeClassA<F>
    {
        self.calc.param()
    }
    pub fn param_mut(&mut self) -> &mut PentodeClassA<F>
    {
        self.calc.param_mut()
    }

    pub fn calibrate(&mut self)
    {
        [self.offset, _] = self.calc.vp_a(-self.param().v_c);
    }

    pub fn saturate(&mut self, rate: F, x: F) -> F
    {
        // Math: https://www.normankoren.com/Audio/Tubemodspice_article.html

        let one = F::one();
        let zero = F::zero();

        let param = *self.param();

        let vg = self.input_filter.vg(param, rate, x);

        let [vp, a] = self.calc.vp_a(vg);

        let y = vp - self.offset;

        self.miller_effect = one + a.max(zero);

        self.input_filter.update_miller_effect_input(self.miller_effect);
        self.output_filter.update_miller_effect_output(self.miller_effect);

        self.output_filter.y(rate, y)
    }

    pub fn miller_effect(&self) -> F
    {
        self.miller_effect
    }
    pub fn offset(&self) -> F
    {
        self.offset
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
        const RANGE: Range<f32> = -2.0..50.0;
        const RATE: f32 = 8000.0;
        const DY: f32 = 0.001;
        
        let param = PentodeClassA {
            r_i: 1e3,
            r_p: 1e3,
            v_pp: 24.0,
            v_g2: 3.3,
            v_c: 0.0
        };
        
        let mut t0 = Pentode::<_, _>::new(param.cache(DY), Tube6L6CG);
        let mut t1 = Pentode::<_, _>::new(param.cache(DY), Tube6550);
        let mut t2 = Pentode::<_, _>::new(param.cache(DY), TubeKT88);

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