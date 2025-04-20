use core::ops::RangeFull;
use alloc::{vec::Vec, alloc::{Allocator, Global}};

use num::Float;

use crate::SaturationMut;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheTable<F, const N: usize, Y, A = Global>
where
    F: Float,
    Y: FnMut(F) -> [F; N],
    A: Allocator + Clone
{
    func: Y,
    has_changed: bool,
    dx: F,
    infinity: [[F; N]; 2],
    curve: [Vec<[F; N], A>; 2]
}

impl<F, const N: usize, Y> CacheTable<F, N, Y>
where
    F: Float,
    Y: FnMut(F) -> [F; N]
{
    pub fn new(func: Y, dx: F) -> Self
    {
        Self::new_in(func, dx, Global)
    }
    
}

impl<F, const N: usize, Y, A> CacheTable<F, N, Y, A>
where
    F: Float,
    Y: FnMut(F) -> [F; N],
    A: Allocator + Clone
{
    const MAX: usize = u32::MAX as usize;

    pub fn new_in(mut func: Y, dx: F, alloc: A) -> Self
    {
        let (neg_inf, inf) = Self::inf(dx);
        let infinity = [func(neg_inf), func(inf)];
        Self {
            func,
            has_changed: false,
            dx: dx.abs(),
            infinity,
            curve: [Vec::new_in(alloc.clone()), Vec::new_in(alloc)]
        }
    }

    pub fn func(&self) -> &Y
    {
        &self.func
    }
    pub fn func_mut(&mut self) -> &mut Y
    {
        self.has_changed = true;
        &mut self.func
    }

    pub fn reset(&mut self)
    {
        for curve in self.curve.iter_mut()
        {
            curve.clear()
        }
        self.infinity = [(self.func)(F::neg_infinity()), (self.func)(F::infinity())];
    }

    fn max(dx: F) -> Option<F>
    {
        F::from(Self::MAX)
            .map(|i| i*dx)
    }

    fn inf(dx: F) -> (F, F)
    {
        if let Some(max) = Self::max(dx)
        {
            (-max, max)
        }
        else
        {
            (F::neg_infinity(), F::infinity())
        }
    }

    fn is_bounded(&self, x: F) -> bool
    {
        if let Some(max) = Self::max(self.dx)
        {
            x.abs() < max
        }
        else
        {
            false
        }
    }

    fn index(&self, x: F) -> Option<(usize, usize, F, F)>
    {
        if self.is_bounded(x)
        {
            let mut p = x.abs()/self.dx;
            if let (Some(i0), Some(i1)) = (
                p.floor().to_usize(),
                p.ceil().to_usize()
            )
            {
                p = p.fract();
                let q = F::one() - p;
                return Some((
                    i0,
                    i1,
                    q,
                    p
                ))
            }
        }
        None
    }

    pub fn saturate(&mut self, x: F) -> [F; N]
    {
        let b = x.is_sign_positive();

        self.index(x)
            .and_then(|(i0, i1, q, p)| {
                let curve = &mut self.curve[b as usize];

                let l = curve.len();
                let l2 = i1 + 1;
                curve.reserve(l2.saturating_sub(l));
                for i in l..l2
                {
                    if let Some(i) = F::from(i)
                    {
                        let mut x = i*self.dx;
                        if !b
                        {
                            x = -x
                        }
                        let y = (self.func)(x);
                        if y == self.infinity[b as usize]
                        {
                            return Some(y)
                        }
                        curve.push(y);
                    }
                    else
                    {
                        return None
                    }
                }

                let mut y = unsafe {
                    *curve.get_unchecked(i0)
                };
                let y1 = unsafe {
                    *curve.get_unchecked(i1)
                };

                for (y, y1) in y.iter_mut()
                    .zip(y1)
                {
                    *y = *y*q + y1*p;
                }
                
                Some(y)
            }).unwrap_or_else(|| self.infinity[b as usize])
    }
}

impl<F, Y, A> SaturationMut<F, RangeFull> for CacheTable<F, 1, Y, A>
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