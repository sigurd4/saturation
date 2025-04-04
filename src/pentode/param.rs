use real_time_fir_iir_filters::param::FilterFloat;

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