#[cfg(feature = "alloc")]
use alloc::alloc::Allocator;

use real_time_fir_iir_filters::param::FilterFloat;

#[cfg(feature = "alloc")]
use super::{PentodeCache, PentodeModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PentodeClassA<F>
where
    F: FilterFloat
{
    /// Input resistor
    pub r_i: F,
    /// Plate resistor
    pub r_p: F,
    /// Screen gate voltage
    pub v_g2: F,
    /// Supply voltage
    pub v_pp: F,
    /// Cathode voltage
    pub v_c: F
}
#[cfg(feature = "alloc")]
impl<F> PentodeClassA<F>
where
    F: FilterFloat
{
    pub fn cache_in<M, A>(self, dy: F, alloc: A) -> PentodeCache<F, M, A>
    where
        M: PentodeModel,
        A: Allocator + Clone
    {
        PentodeCache::new_in(self, dy, alloc)
    }

    pub fn cache<M>(self, dy: F) -> PentodeCache<F, M>
    where
        M: PentodeModel
    {
        PentodeCache::new(self, dy)
    }
}