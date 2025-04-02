use crate::{PentodeModel, TriodeModel};

// Triodes:

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Tube6DJ8;
impl TriodeModel for Tube6DJ8
{
    const MU: f64 = 28.0;
    const EX: f64 = 1.3;
    const K_G1: f64 = 330.0;
    const K_P: f64 = 320.0;
    const K_VB: f64 = 300.0;
    const C_CG: f64 = 2.3e-12;
    const C_PG: f64 = 2.1e-12;
    const C_CP: f64 = 0.7e-12;
    const R_GI: f64 = 2e3;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Tube12AX7;
impl TriodeModel for Tube12AX7
{
    const MU: f64 = 8.7;
    const EX: f64 = 1.35;
    const K_G1: f64 = 1460.0;
    const K_P: f64 = 48.0;
    const K_VB: f64 = 12.0;
    const C_CG: f64 = 14e-12;
    const C_PG: f64 = 850e-15;
    const C_CP: f64 = 12e-12;
    const R_GI: f64 = 1e3;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Tube12AU7;
impl TriodeModel for Tube12AU7
{
    const MU: f64 = 100.0;
    const EX: f64 = 1.4;
    const K_G1: f64 = 1060.0;
    const K_P: f64 = 600.0;
    const K_VB: f64 = 300.0;
    const C_CG: f64 = 2.3e-12;
    const C_PG: f64 = 2.4e-12;
    const C_CP: f64 = 900e-15;
    const R_GI: f64 = 2e3;
}

// Pentodes:

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Tube6L6CG;
impl TriodeModel for Tube6L6CG
{
    const MU: f64 = 21.5;
    const EX: f64 = 1.3;
    const K_G1: f64 = 1180.0;
    const K_P: f64 = 84.0;
    const K_VB: f64 = 300.0;
    const C_CG: f64 = 2.3e-12;
    const C_PG: f64 = 2.2e-12;
    const C_CP: f64 = 1e-12;
    const R_GI: f64 = 2e3;
}
impl PentodeModel for Tube6L6CG
{
    const K_G2: f64 = 4500.0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Tube6550;
impl TriodeModel for Tube6550
{
    const MU: f64 = 7.9;
    const EX: f64 = 1.35;
    const K_G1: f64 = 890.0;
    const K_P: f64 = 60.0;
    const K_VB: f64 = 24.0;
    const C_CG: f64 = 14e-12;
    const C_PG: f64 = 850e-15;
    const C_CP: f64 = 12e-12;
    const R_GI: f64 = 1e3;
}
impl PentodeModel for Tube6550
{
    const K_G2: f64 = 4800.0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct TubeKT88;
impl TriodeModel for TubeKT88
{
    const MU: f64 = 8.8;
    const EX: f64 = 1.35;
    const K_G1: f64 = 730.0;
    const K_P: f64 = 32.0;
    const K_VB: f64 = 16.0;
    const C_CG: f64 = 14e-12;
    const C_PG: f64 = 850e-15;
    const C_CP: f64 = 12e-12;
    const R_GI: f64 = 1e3;
}
impl PentodeModel for TubeKT88
{
    const K_G2: f64 = 4200.0;
}