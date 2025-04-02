use real_time_fir_iir_filters::rtf::{Rtf, RtfBase};

pub trait Rtf1: RtfBase<OUTPUTS = 1>
{
    fn filter(&mut self, rate: Self::F, x: Self::F) -> Self::F;
}
impl<T> Rtf1 for T
where
    Self: Rtf<OUTPUTS = 1>,
    [(); Self::OUTPUTS]:
{
    fn filter(&mut self, rate: Self::F, x: Self::F) -> Self::F
    {
        unsafe {
            *Rtf::filter(self, rate, x).first().unwrap_unchecked()
        }
    }
}