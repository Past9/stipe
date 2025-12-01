use crate::{
    bdd::{Bdd, TyAtom},
    ty::TyConfig,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product<'a, C, T>(pub &'a Bdd<'a, C, T>, pub &'a Bdd<'a, C, T>)
where
    C: TyConfig,
    T: TyAtom;
impl<'a, C, T> TyAtom for Product<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
}
