use core::ops::{Range, RangeFull};
use alloc::{vec::Vec, alloc::{Allocator, Global}};

use num::Float;

use crate::SaturateMut;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CacheTableCurve<F, const N: usize, A = Global>
where
    F: Float,
    A: Allocator
{
    infinity: [[F; N]; 2],
    curve: Vec<[F; N], A>,
    range: Range<F>
}

impl<F, const N: usize, A> CacheTableCurve<F, N, A>
where
    F: Float,
    A: Allocator
{
    const MAX: usize = u16::MAX as usize - 1;

    fn new_in<Y>(mut func: Y, mut range: Range<F>, resolution: usize, alloc: A) -> Self
    where
        Y: FnMut(F) -> [F; N]
    {
        let (neg_inf, inf) = Self::inf(&range, resolution);

        let x_bound @ [x_min, x_max] = [range.start.max(neg_inf), range.end.min(inf)];
        let infinity = x_bound.map(&mut func);
        range = x_min..x_max;

        Self {
            infinity,
            curve: Vec::new_in(alloc),
            range
        }
    }

    fn reset(&mut self)
    {
        self.curve.clear();
    }
    fn is_set(&self, resolution: usize) -> bool
    {
        self.curve.len() == resolution + 1
    }

    fn refresh<Y>(&mut self, mut func: Y, resolution: usize)
    where
        Y: FnMut(F) -> [F; N]
    {
        self.reset();

        let one = F::one();
        let (neg_inf, inf) = Self::inf(&self.range, resolution);

        let x_bound @ [x_min, x_max] = [self.range.start.max(neg_inf), self.range.end.min(inf)];
        self.infinity = x_bound.map(&mut func);

        (0..=resolution)
            .map(|i| {
                if let Some(p) = F::from(i as f64/resolution as f64)
                {
                    let q = one - p;
                    return x_min*q + x_max*p
                }

                x_bound[(i > resolution/2) as usize]
            })
            .map(|x| func(x))
            .collect_into(&mut self.curve);
    }

    fn max(dx: F) -> Option<F>
    {
        F::from(Self::MAX)
            .map(|i| i*dx)
    }

    fn dx(range: &Range<F>, resolution: usize) -> F
    {
        if let Some(r) = F::from(resolution)
        {
            let dx = ((range.end - range.start)/r).abs();
            if dx.is_finite()
            {
                return dx
            }
        }

        F::zero()
    }

    fn inf(range: &Range<F>, resolution: usize) -> (F, F)
    {
        if resolution >= Self::MAX
        {
            let dx = Self::dx(range, resolution);
            if let Some(max) = Self::max(dx)
            {
                return (-max, max)
            }
        }
        (F::neg_infinity(), F::infinity())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheTable<F, const N: usize, Y, A = Global>
where
    F: Float,
    Y: FnMut(F) -> [F; N],
    A: Allocator
{
    func: Y,
    resolution: usize,
    curve: CacheTableCurve<F, N, A>
}

impl<F, const N: usize, Y> CacheTable<F, N, Y>
where
    F: Float,
    Y: FnMut(F) -> [F; N]
{
    pub fn new(func: Y, range: Range<F>, resolution: usize) -> Self
    {
        Self::new_in(func, range, resolution, Global)
    }
    
}

impl<F, const N: usize, Y, A> CacheTable<F, N, Y, A>
where
    F: Float,
    Y: FnMut(F) -> [F; N],
    A: Allocator + Clone
{
    pub fn new_in(mut func: Y, range: Range<F>, mut resolution: usize, alloc: A) -> Self
    {
        resolution = resolution.min(CacheTableCurve::<F, N, A>::MAX);
        let curve = CacheTableCurve::new_in(&mut func, range, resolution, alloc);

        Self {
            func,
            resolution,
            curve
        }
    }

    pub fn func(&self) -> &Y
    {
        &self.func
    }
    pub fn func_mut(&mut self) -> &mut Y
    {
        self.reset();
        &mut self.func
    }

    pub fn reset(&mut self)
    {
        self.curve.reset()
    }

    fn is_bounded(&self, x: F) -> bool
    {
        self.curve.is_set(self.resolution) && self.curve.range.start <= x && x <= self.curve.range.end
    }

    fn index(&self, x: F) -> Option<(usize, usize, F, F)>
    {
        if self.is_bounded(x) && let Some(r) = F::from(self.resolution)
        {
            let z = r*(x - self.curve.range.start)/(self.curve.range.end - self.curve.range.start);
            if let (Some(i0), Some(i1)) = (
                z.floor().to_usize(),
                z.ceil().to_usize()
            )
            {
                let p = z.fract();
                let q = F::one() - p;

                return Some((i0, i1, p, q))
            }
        }
        None
    }

    pub fn saturate(&mut self, x: F) -> [F; N]
    {
        if !self.curve.is_set(self.resolution)
        {
            self.curve.refresh(&mut self.func, self.resolution);
        }
        self.index(x)
            .and_then(|(i0, i1, q, p)| {
                let mut y = unsafe {
                    *self.curve.curve.get_unchecked(i0)
                };
                let y1 = unsafe {
                    *self.curve.curve.get_unchecked(i1)
                };

                for (y, y1) in y.iter_mut()
                    .zip(y1)
                {
                    *y = *y*q + y1*p;
                }
                
                Some(y)
            }).unwrap_or_else(|| self.curve.infinity[x.is_sign_positive() as usize])
    }
}

impl<F, Y, A> SaturateMut<F, RangeFull> for CacheTable<F, 1, Y, A>
where
    F: Float,
    Y: FnMut(F) -> [F; 1],
    A: Allocator + Clone
{
    fn saturate_mut(&mut self, x: F, RangeFull: RangeFull) -> F
    {
        let [y] = self.saturate(x);
        y
    }
}