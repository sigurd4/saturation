use crate::DiodeModel;

pub struct Diode1N4148;
impl DiodeModel for Diode1N4148
{
    const K: f64 = 1.38e-23;
    const I_0: f64 = 4e-9;
    const Q_E: f64 = 1.602176634e-19;
    const ETA: f64 = 2.0;
    const T: f64 = 20.0 + 273.15;
}