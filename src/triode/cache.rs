use core::{marker::PhantomData, ops::Range};
use alloc::alloc::{Allocator, Global};

use real_time_fir_iir_filters::param::FilterFloat;

use crate::CacheTable;

use super::{calc::TriodeCalc, TriodeClassA, TriodeModel};

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct TriodeCacheFunc<F, M>
where
    F: FilterFloat,
    M: TriodeModel
{
    param: TriodeClassA<F>,
    marker: PhantomData<M>
}
impl<F, M> FnOnce<(F,)> for TriodeCacheFunc<F, M>
where
    F: FilterFloat,
    M: TriodeModel
{
    type Output = [F; 2];

    extern "rust-call" fn call_once(mut self, (vg,): (F,)) -> Self::Output
    {
        TriodeCalc::<F, M>::vp_a(&mut self.param, vg)
    }
}
impl<F, M> FnMut<(F,)> for TriodeCacheFunc<F, M>
where
    F: FilterFloat,
    M: TriodeModel
{
    extern "rust-call" fn call_mut(&mut self, (vg,): (F,)) -> Self::Output
    {
        TriodeCalc::<F, M>::vp_a(&mut self.param, vg)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriodeCache<F, M, A = Global>
where
    F: FilterFloat,
    M: TriodeModel,
    A: Allocator + Clone
{
    cache: CacheTable<F, 2, TriodeCacheFunc<F, M>, A>
}
impl<F, M> TriodeCache<F, M>
where
    F: FilterFloat,
    M: TriodeModel
{
    pub fn new(param: TriodeClassA<F>, range: Range<F>, resolution: usize) -> Self
    {
        Self::new_in(param, range, resolution, Global)
    }
}
impl<F, M, A> TriodeCache<F, M, A>
where
    F: FilterFloat,
    M: TriodeModel,
    A: Allocator + Clone
{
    pub fn new_in(param: TriodeClassA<F>, range: Range<F>, resolution: usize, alloc: A) -> Self
    {
        Self {
            cache: CacheTable::new_in(
                TriodeCacheFunc {
                    param,
                    marker: PhantomData
                },
                range,
                resolution,
                alloc
            )
        }
    }
}
impl<F, M, A> TriodeCalc<F, M> for TriodeCache<F, M, A>
where
    F: FilterFloat,
    M: TriodeModel,
    A: Allocator + Clone
{
    fn param(&self) -> &TriodeClassA<F>
    {
        &self.cache.func().param
    }
    fn param_mut(&mut self) -> &mut TriodeClassA<F>
    {
        &mut self.cache.func_mut().param
    }
    fn vp_a(&mut self, vg: F) -> [F; 2]
    {
        self.cache.saturate(vg)
    }
}