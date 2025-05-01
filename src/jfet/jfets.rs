use super::JFETModel;

pub struct JFET2N5458;

impl JFETModel for JFET2N5458
{
    const BETA: f64 = 488.9e-6;
    const LAMBDA: f64 = 3.167e-3;
    const LAMBDA1: f64 = Self::VTO;
    const VTO: f64 = -2.882;
}