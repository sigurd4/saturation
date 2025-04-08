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

pub trait Rtf2: RtfBase<OUTPUTS = 2>
{
    fn filter(&mut self, rate: Self::F, x: Self::F) -> [Self::F; 2];
}
impl<T> Rtf2 for T
where
    Self: Rtf<OUTPUTS = 2>,
    [(); Self::OUTPUTS]:
{
    fn filter(&mut self, rate: Self::F, x: Self::F) -> [Self::F; 2]
    {
        unsafe {
            core::intrinsics::transmute_unchecked(Rtf::filter(self, rate, x))
        }
    }
}