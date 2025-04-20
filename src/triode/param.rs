#[cfg(feature = "alloc")]
use alloc::alloc::Allocator;

use num::Float;

#[cfg(feature = "alloc")]
use real_time_fir_iir_filters::param::FilterFloat;

#[cfg(feature = "alloc")]
use super::{TriodeModel, TriodeCache};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TriodeClassA<F>
where
    F: Float
{
    /// Input resistor
    pub r_i: F,
    /// Plate resistor
    pub r_p: F,
    /// Supply voltage
    pub v_pp: F,
    /// Cathode voltage
    pub v_c: F
}
#[cfg(feature = "alloc")]
impl<F> TriodeClassA<F>
where
    F: FilterFloat
{
    pub fn cache_in<M, A>(self, dy: F, alloc: A) -> TriodeCache<F, M, A>
    where
        M: TriodeModel,
        A: Allocator + Clone
    {
        TriodeCache::new_in(self, dy, alloc)
    }

    pub fn cache<M>(self, dy: F) -> TriodeCache<F, M>
    where
        M: TriodeModel
    {
        TriodeCache::new(self, dy)
    }
}