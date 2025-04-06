use core::{cmp::Ordering, fmt::Debug, ops::{Add, Deref, Div, Mul, Rem, Sub}};

use num::Float;

#[derive(Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Finite<F>(F)
where
    F: Float;
impl<F> Finite<F>
where
    F: Float
{
    pub fn new(value: F) -> Option<Self>
    {
        if value.is_finite()
        {
            Some(Self(value))
        }
        else
        {
            None
        }
    }
}
impl<F> Debug for Finite<F>
where
    F: Float + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.0.fmt(f)
    }
}
impl<F> PartialEq<F> for Finite<F>
where
    F: Float
{
    fn eq(&self, other: &F) -> bool
    {
        &**self == other
    }
    fn ne(&self, other: &F) -> bool
    {
        &**self != other
    }
}
impl<F> PartialOrd<F> for Finite<F>
where
    F: Float
{
    fn partial_cmp(&self, other: &F) -> Option<Ordering>
    {
        (**self).partial_cmp(other)
    }

    fn lt(&self, other: &F) -> bool
    {
        (**self).lt(other)
    }

    fn le(&self, other: &F) -> bool
    {
        (**self).le(other)
    }

    fn gt(&self, other: &F) -> bool
    {
        (**self).gt(other)
    }

    fn ge(&self, other: &F) -> bool
    {
        (**self).ge(other)
    }
}
impl<F> Eq for Finite<F>
where
    F: Float
{
    
}
impl<F> Ord for Finite<F>
where
    F: Float
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        unsafe {
            self.partial_cmp(other).unwrap_unchecked()
        }
    }
}
impl<F> Deref for Finite<F>
where
    F: Float
{
    type Target = F;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

macro_rules! binop {
    ($($trait:ident :: $f:ident)*) => {
        $(
            impl<F> $trait<F> for Finite<F>
            where
                F: Float
            {
                type Output = F;
            
                #[inline]
                fn $f(self, rhs: F) -> Self::Output
                {
                    (*self).$f(rhs)
                }
            }
            impl<F> $trait for Finite<F>
            where
                F: Float
            {
                type Output = F;
            
                #[inline]
                fn $f(self, rhs: Self) -> Self::Output
                {
                    self.$f(*rhs)
                }
            }
        )*
    };
}

binop!(
    Add::add
    Sub::sub
    Mul::mul
    Div::div
    Rem::rem
);