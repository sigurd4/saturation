#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]

use num::Float;

moddef::moddef!(
    flat(pub) mod {
        diode for cfg(feature = "diodes"),
        pentode for cfg(feature = "tubes"),
        triode for cfg(feature = "tubes"),
        soft_exp for cfg(feature = "soft_exp"),
    },
    pub mod {
        diodes for cfg(feature = "diodes"),
        tubes for cfg(feature = "tubes")
    },
    mod {
        plot for cfg(test),
        rtf for cfg(feature = "tubes")
    }
);

pub use real_time_fir_iir_filters::f;

pub trait Saturation<F, R>
where
    F: Float
{
    fn saturate(&self, x: F, range: R) -> F;
}

fn exp_ln_1p<F>(x: F) -> F
where
    F: Float
{
    x.max(F::zero()) + (-x.abs()).exp().ln_1p()
}

fn lambertw<F>(x: F) -> F
where
    F: Float
{
    const THRESHOLD: f64 = 2.26445;
    const C: [f64; 2] = [1.0, 1.546865557];
    const D: [f64; 2] = [0.0, 2.250366841];
    const A: [f64; 2] = [0.0, -0.737769969];

    let threshold = F::from(THRESHOLD).unwrap();
    let b = x < threshold;
    let c = f!(C[b as usize]);
    let d = f!(D[b as usize]);
    let a = f!(A[b as usize]);

    let one = F::one();
    let two = one + one;
    let four = two + two;

    let logterm = (c * x + d).ln();
    let loglogterm = logterm.ln();

    let minusw = -a - logterm + loglogterm - loglogterm / logterm;
    let expminusw = minusw.exp();
    let xexpminusw = x*expminusw;
    let pexpminusw = xexpminusw - minusw;

    (two*xexpminusw - minusw*(four*xexpminusw - minusw*pexpminusw))
        / (two + pexpminusw*(two - minusw))
}

fn change<F>(rate: F) -> F
where
    F: Float
{
    f!(1.0e-3)/(F::one() + rate)
}

#[cfg(test)]
mod tests
{
    use core::ops::Range;

    use linspace::LinspaceArray;

    const PLOT_TARGET: &str = "plots";

    pub fn plot<const N: usize, F>(sat_name: &str, range: Range<f32>, mut f: F)
    where
        F: FnMut(f32) -> [f32; N]
    {
        const RES: usize = 512;

        let x: [f32; RES] = range.linspace_array();
        
        let mut first = true;
        let file_name_no_extension: String = sat_name
            .chars()
            .flat_map(|c| {
                if c.is_ascii_uppercase()
                {
                    if first
                    {
                        first = false;
                        vec![c.to_ascii_lowercase()]
                    }
                    else
                    {
                        vec!['_', c.to_ascii_lowercase()]
                    }
                }
                else
                {
                    vec![c]
                }
            })
            .collect();

        let mut y = [[0.0; RES]; N];
        for (i, yy) in x.iter()
            .map(|&x| f(x))
            .enumerate()
        {
            for (src, dst) in yy.into_iter()
                .zip(y.iter_mut())
            {
                dst[i] = src;
            }
        }

        crate::plot::plot_curves(
            &format!("Curve of {}", sat_name),
            &format!("{}/{}.png", PLOT_TARGET, file_name_no_extension),
            [x; N],
            y
        ).expect("Plot failed");
    }
}
