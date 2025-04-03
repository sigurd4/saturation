pub trait DiodeModel
{
    const K: f64;
    const I_0: f64;
    const Q_E: f64;
    const ETA: f64;
    const T: f64;
}