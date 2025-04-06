use core::marker::PhantomData;
use std::alloc::{Allocator, Global};

use real_time_fir_iir_filters::param::FilterFloat;

use crate::CurveCache;

use super::{calc::PentodeCalc, PentodeClassA, PentodeModel};

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct PentodeCacheFunc<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    param: PentodeClassA<F>,
    marker: PhantomData<M>
}
impl<F, M> FnOnce<(F,)> for PentodeCacheFunc<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    type Output = [F; 2];

    extern "rust-call" fn call_once(mut self, (vg,): (F,)) -> Self::Output
    {
        PentodeCalc::<F, M>::vp_a(&mut self.param, vg)
    }
}
impl<F, M> FnMut<(F,)> for PentodeCacheFunc<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    extern "rust-call" fn call_mut(&mut self, (vg,): (F,)) -> Self::Output
    {
        PentodeCalc::<F, M>::vp_a(&mut self.param, vg)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PentodeCache<F, M, A = Global>
where
    F: FilterFloat,
    M: PentodeModel,
    A: Allocator + Clone
{
    cache: CurveCache<F, 2, PentodeCacheFunc<F, M>, A>
}
impl<F, M> PentodeCache<F, M>
where
    F: FilterFloat,
    M: PentodeModel
{
    pub fn new(param: PentodeClassA<F>, slope: F) -> Self
    {
        Self::new_in(param, slope, Global)
    }
}
impl<F, M, A> PentodeCache<F, M, A>
where
    F: FilterFloat,
    M: PentodeModel,
    A: Allocator + Clone
{
    pub fn new_in(param: PentodeClassA<F>, dy: F, alloc: A) -> Self
    {
        Self {
            cache: CurveCache::new_in(
                PentodeCacheFunc {
                    param,
                    marker: PhantomData
                },
                dy,
                alloc
            )
        }
    }
}
impl<F, M, A> PentodeCalc<F, M> for PentodeCache<F, M, A>
where
    F: FilterFloat,
    M: PentodeModel,
    A: Allocator + Clone
{
    fn param(&self) -> &PentodeClassA<F>
    {
        &self.cache.func().param
    }
    fn param_mut(&mut self) -> &mut PentodeClassA<F>
    {
        &mut self.cache.func_mut().param
    }
    fn vp_a(&mut self, vg: F) -> [F; 2]
    {
        self.cache.saturate(vg)
    }
}