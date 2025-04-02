use crate::TriodeModel;

pub trait PentodeModel: TriodeModel
{
    const K_G2: f64;
}