use real_time_fir_iir_filters::{conf::LowPass, filters::iir::first::FirstOrderRCFilter, param::FilterFloat};

use crate::tubes::Tube12AX7;

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
pub struct Triode<F, M = Tube12AX7, FI = FirstOrderRCFilter<LowPass, F>, FO = FirstOrderRCFilter<LowPass, F>, C = TriodeCache<F, M>>
where
    F: FilterFloat,
    M: TriodeModel,
    C: TriodeCalc<F, M>,
    FI: TriodeFilter<F, M>,
    FO: TriodeFilter<F, M>
{
    calc: C,
    input_filter: FI,
    output_filter: FO,
    miller_effect: F,
    offset: F,
    model: M
}

impl<F, M, C, FI, FO> Triode<F, M, FI, FO, C>
where
    F: FilterFloat,
    M: TriodeModel,
    C: TriodeCalc<F, M>,
    FI: TriodeFilter<F, M>,
    FO: TriodeFilter<F, M>
{
    pub fn new(calc: C, model: M) -> Self
    {
        let input_filter = FI::new_input_filter(calc.param().r_i);
        let output_filter = FO::new_output_filter(calc.param().r_p);
        let mut triode = Self {
            calc,
            input_filter,
            output_filter,
            miller_effect: F::one(),
            offset: F::zero(),
            model,
        };
        triode.calibrate();
        triode
    }

    pub fn param(&self) -> &TriodeClassA<F>
    {
        self.calc.param()
    }
    pub fn param_mut(&mut self) -> &mut TriodeClassA<F>
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

        self.miller_effect = one - a.min(zero);
        let change = crate::change(rate);

        self.input_filter.update_miller_effect(self.miller_effect, change);
        self.output_filter.update_miller_effect(self.miller_effect.recip(), change);

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

    use crate::tubes::{Tube12AU7, Tube6550, Tube6DJ8, Tube6L6CG, TubeKT88};

    use super::*;

    #[test]
    fn it_works()
    {
        const RANGE: Range<f32> = -20.0..20.0;
        const RATE: f32 = 8000.0;
        const DY: f32 = 0.001;
        
        let param = TriodeClassA {
            r_i: 1e3,
            r_p: 47e3,
            v_pp: 24.0,
            v_c: 0.0
        };
        
        let mut t0 = Triode::<_, _, (), ()>::new(param.cache(DY), Tube6DJ8);
        let mut t1 = Triode::<_, _, (), ()>::new(param.cache(DY), Tube12AX7);
        let mut t2 = Triode::<_, _, (), ()>::new(param.cache(DY), Tube12AU7);
        let mut t3 = Triode::<_, _, (), ()>::new(param.cache(DY), Tube6L6CG);
        let mut t4 = Triode::<_, _, (), ()>::new(param.cache(DY), Tube6550);
        let mut t5 = Triode::<_, _, (), ()>::new(param.cache(DY), TubeKT88);

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