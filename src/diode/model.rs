pub trait DiodeModel
{
    // Peak reverse current
    const I_0: f64;
    /// Ideality factor
    const ETA: f64;
}