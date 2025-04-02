pub trait TriodeModel
{
    const MU: f64;
    const EX: f64;
    const K_G1: f64;
    const K_P: f64;
    const K_VB: f64;
    const C_CG: f64;
    const C_PG: f64;
    const C_CP: f64;
    const R_GI: f64;
}