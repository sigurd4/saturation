use crate::DiodeModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Diode1N4148;
impl DiodeModel for Diode1N4148
{
    const I_0: f64 = 7e-9;
    const ETA: f64 = 2.0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Diode1N914;
impl DiodeModel for Diode1N914
{
    const I_0: f64 = 25e-9;
    const ETA: f64 = 1.752;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Diode1N4001;
impl DiodeModel for Diode1N4001
{
    const I_0: f64 = 18.8e-9;
    const ETA: f64 = 1.9;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Diode1N34A;
impl DiodeModel for Diode1N34A
{
    const I_0: f64 = 1e-3;
    const ETA: f64 = 1.3;
}