use core::marker::PhantomData;

use num::Float;
use crate::f;

moddef::moddef!(
    flat(pub) mod {
        diode_clipper,
        model
    },
    pub mod {
        diodes
    }
);