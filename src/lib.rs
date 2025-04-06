#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]
#![feature(specialization)]
#![feature(allocator_api)]
#![feature(btreemap_alloc)]
#![feature(btree_cursors)]
#![feature(negative_impls)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use core::ops::RangeBounds;

use num::Float;

moddef::moddef!(
    flat(pub) mod {
        diode for cfg(feature = "diodes"),
        pentode for cfg(feature = "tubes"),
        triode for cfg(feature = "tubes"),

        atanmoid,
        curve_cache,
        erfmoid for cfg(feature = "libm"),
        linmoid,
        pythmoid,
        sinh_atanmoid,
        soft_exp for cfg(feature = "soft_exp"),
        tanh
    },
    pub mod {
        diodes for cfg(feature = "diodes"),
        tubes for cfg(feature = "tubes")
    },
    mod {
        finite,
        plot for cfg(test),
        rtf for cfg(feature = "tubes")
    }
);

#[allow(unused)]
#[macro_export]
macro_rules! f {
    ($x:expr; $($f:tt)*) => {
        <$($f)* as num::NumCast>::from($x).unwrap()
    };
    ($x:expr) => {
        f!($x; F)
    };
}

pub trait SaturationMut<F, R>
where
    F: Float,
    R: RangeBounds<F>
{
    fn saturate_mut(&mut self, x: F, range: R) -> F;
}

pub trait Saturation<F, R>: SaturationMut<F, R>
where
    F: Float,
    R: RangeBounds<F>
{
    fn saturate(&self, x: F, range: R) -> F;
}

#[cfg(feature = "tubes")]
fn exp_ln_1p<F>(x: F) -> F
where
    F: Float
{
    //x.exp().ln_1p()
    x.max(F::zero()) + (-x.abs()).exp().ln_1p()
}

#[cfg(feature = "diodes")]
fn lambertw<F>(x_ln: F) -> F
where
    F: Float
{
    const THRESHOLD: f64 = 0.8173319038410221;
    const C: f64 = 1.546865557;
    const D: f64 = 2.250366841;
    const A: [f64; 2] = [0.0, -0.737769969];

    let threshold = F::from(THRESHOLD).unwrap();
    let b = x_ln < threshold;
    let a = f!(A[b as usize]);

    let one = F::one();
    let two = one + one;
    let four = two + two;

    let logterm = if b { (f!(C) * x_ln.exp() + f!(D)).ln() } else { x_ln };
    let loglogterm = logterm.ln();

    let minusw = -a - logterm + loglogterm - loglogterm / logterm;
    let xexpminusw = (x_ln + minusw).exp();
    let minusw2 = minusw * minusw;
    let minusw3 = minusw2 * minusw;

    //(two*xexpminusw - minusw*(four*xexpminusw - minusw*pexpminusw))
    // (two + pexpminusw*(two - minusw))
    (two - minusw * four + minusw2 - minusw3 / xexpminusw) / ((two - minusw * two + minusw2) / xexpminusw + two - minusw)
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
        for (i, yy) in x.iter().map(|&x| f(x)).enumerate()
        {
            for (src, dst) in yy.into_iter().zip(y.iter_mut())
            {
                dst[i] = src;
            }
        }

        crate::plot::plot_curves(
            &format!("Curve of {}", sat_name),
            &format!("{}/{}.png", PLOT_TARGET, file_name_no_extension),
            [x; N],
            y
        )
        .expect("Plot failed");
    }
}
