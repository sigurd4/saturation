use real_time_fir_iir_filters::{conf::LowPass, filters::iir::first::FirstOrderRCFilter, param::FilterFloat};

use crate::tubes::Tube6550;

moddef::moddef!(
    flat(pub) mod {
        cache for cfg(feature = "alloc"),
        model,
        param
    },
    flat mod {
        calc,
        filter
    }
);

macro_rules! decl {
    ($calc:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Pentode<F, M = Tube6550, FI = FirstOrderRCFilter<LowPass, F>, FO = FirstOrderRCFilter<LowPass, F>, FC = FirstOrderRCFilter<LowPass, F>, C = $calc>
        where
            F: FilterFloat,
            M: PentodeModel,
            C: PentodeCalc<F, M>,
            FI: PentodeFilter<F, M>,
            FO: PentodeFilter<F, M>,
            FC: PentodeCathodeFilter<F, M>
        {
            calc: C,
            input_filter: FI,
            output_filter: FO,
            cathode_filter: FC,
            miller_effect: F,
            offset: F,
            model: M
        }
    };
}
#[cfg(feature = "alloc")]
decl!(PentodeCache<F, M>);
#[cfg(not(feature = "alloc"))]
decl!(PentodeClassA<F>);

impl<F, M, C, FI, FO, FC> Pentode<F, M, FI, FO, FC, C>
where
    F: FilterFloat,
    M: PentodeModel,
    C: PentodeCalc<F, M>,
    FI: PentodeFilter<F, M>,
    FO: PentodeFilter<F, M>,
    FC: PentodeCathodeFilter<F, M>
{
    pub fn new(calc: C, model: M, cathode: FC::Param) -> Self
    {
        let param = calc.param();
        let input_filter = FI::new_input_filter(param.r_i);
        let output_filter = FO::new_output_filter(param.r_p);
        let cathode_filter = FC::new_cathode_filter(cathode);
        let mut pentode = Self {
            calc,
            input_filter,
            output_filter,
            cathode_filter,
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
    pub fn param_cathode(&self) -> &FC::Param
    {
        self.cathode_filter.param_cathode()
    }
    pub fn param_cathode_mut(&mut self) -> &mut FC::Param
    {
        self.cathode_filter.param_cathode_mut()
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

        let mut vg = self.cathode_filter.vg_cathode(param, self.miller_effect, rate, x);
        vg = self.input_filter.vg(param, rate, vg);

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

    use real_time_fir_iir_filters::param::RC;

    use crate::tubes::{Tube6550, Tube6L6CG, TubeKT88};

    use super::*;

    #[test]
    fn it_works()
    {
        const RANGE: Range<f32> = -2.0..50.0;
        const RATE: f32 = 8000.0;
        #[cfg(feature = "alloc")]
        const DY: f32 = 0.001;
        
        let param = PentodeClassA {
            r_i: 1e3,
            r_p: 1e3,
            v_pp: 24.0,
            v_g2: 3.3,
            v_c: 0.0
        };
        let param_cathode = RC {
            r: 0.0,
            c: 0.0
        };
        
        #[cfg(feature = "alloc")]
        macro_rules! calc {
            () => {
                param.cache(DY)
            };
        }
        #[cfg(not(feature = "alloc"))]
        macro_rules! calc {
            () => {
                param
            };
        }

        let mut t0 = Pentode::<_, _>::new(calc!(), Tube6L6CG, param_cathode);
        let mut t1 = Pentode::<_, _>::new(calc!(), Tube6550, param_cathode);
        let mut t2 = Pentode::<_, _>::new(calc!(), TubeKT88, param_cathode);

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