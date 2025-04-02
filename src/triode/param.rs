use real_time_fir_iir_filters::param::FilterFloat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TriodeClassA<F>
where
    F: FilterFloat
{
    pub r_i: F,
    pub r_p: F,
    pub v_pp: F
}