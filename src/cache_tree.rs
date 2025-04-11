use core::ops::{Bound, RangeFull};
use std::{alloc::{Allocator, Global}, collections::BTreeMap};

use crate::{f, finite::Finite, SaturationMut};

use num::Float;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheTree<F, const N: usize, Y, A = Global>
where
    F: Float,
    Y: FnMut(F) -> [F; N],
    A: Allocator + Clone
{
    func: Y,
    has_changed: bool,
    infinity: [[F; N]; 2],
    dy_max: F,
    curve: BTreeMap<Finite<F>, [F; N], A>
}

impl<F, const N: usize, Y> CacheTree<F, N, Y>
where
    F: Float,
    Y: FnMut(F) -> [F; N]
{
    pub fn new(func: Y, dy: F) -> Self
    {
        Self::new_in(func, dy, Global)
    }
}
impl<F, const N: usize, Y, A> CacheTree<F, N, Y, A>
where
    F: Float,
    Y: FnMut(F) -> [F; N],
    A: Allocator + Clone
{
    pub fn new_in(mut func: Y, dy: F, alloc: A) -> Self
    {
        let infinity = [func(F::neg_infinity()), func(F::infinity())];
        Self {
            func,
            has_changed: false,
            infinity,
            dy_max: dy.abs(),
            curve: BTreeMap::new_in(alloc)
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
        self.curve.clear();
        self.infinity = [(self.func)(F::neg_infinity()), (self.func)(F::infinity())];
    }

    pub fn saturate(&mut self, x: F) -> [F; N]
    {
        if N == 0
        {
            return core::array::from_fn(|_| F::zero())
        }

        if self.has_changed
        {
            self.reset();
            self.has_changed = false;
        }

        let x = match Finite::new(x)
        {
            Some(x) => x,
            None => return self.infinity[x.is_sign_positive() as usize]
        };
        let mut cursor = self.curve.lower_bound_mut(Bound::Excluded(&x));
        let d = f!(1.0/3.0);
        let mut dxx = None;
        {
            let cursor = cursor.as_cursor();
            let xy0 = cursor.peek_prev();
            let xy1 = cursor.peek_next();
    
            match (xy0, xy1)
            {
                (Some((&x0, y0)), Some((&x1, y1))) => {
                    let dx = x1 - x0;
                    if dx.is_zero()
                    {
                        return *y0
                    }
    
                    let dy = core::array::from_fn::<_, N, _>(|i| y1[i] - y0[i]);
                    let dy_max = unsafe {
                        dy.iter()
                            .copied()
                            .map(|dy| dy.abs())
                            .reduce(F::max)
                            .unwrap_unchecked()
                    };
        
                    if dy_max < self.dy_max
                    {
                        let a = (x - x0)/dx;
                        return core::array::from_fn(|i| y0[i] + dy[i]*a)
                    }
                },
                (None, Some((&x1, y1))) => {
                    if x1 == x
                    {
                        return *y1 
                    }

                    dxx = Some((x - x1)*d);
                },
                (Some((&x0, y0)), None) => {
                    if x0 == x
                    {
                        return *y0 
                    }

                    dxx = Some((x - x0)*d);
                },
                (None, None) => ()
            }
        }

        let y = (self.func)(*x);
        let (xx, yy) = if let Some(dxx) = dxx && let Some(xx) = Finite::new(x + dxx)
        {
            (xx, (self.func)(*xx))
        }
        else
        {
            (x, y)
        };
        cursor.insert_before(xx, yy).unwrap();
        y
    }
}

impl<F, Y, A> SaturationMut<F, RangeFull> for CacheTree<F, 1, Y, A>
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