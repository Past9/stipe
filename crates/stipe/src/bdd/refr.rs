use crate::{
    bdd::{Bdd, TyAtom},
    ty::TyConfig,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Refr<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    pub id: C::TyName,
    pub args: bumpalo::collections::Vec<'a, &'a Bdd<'a, C, T>>,
}
impl<'a, C, T> TyAtom for Refr<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
}
