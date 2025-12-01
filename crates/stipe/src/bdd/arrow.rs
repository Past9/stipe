use crate::{
    bdd::{Bdd, TyAtom},
    ty::TyConfig,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Arrow<'a, C, T>(pub &'a Bdd<'a, C, T>, pub &'a Bdd<'a, C, T>)
where
    C: TyConfig,
    T: TyAtom;
impl<'a, C, T> TyAtom for Arrow<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
}
