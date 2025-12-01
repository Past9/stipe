use crate::{
    bdd::{Bdd, TyAtom},
    ty::{Ty, TyConfig},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Openness {
    Open,
    Closed,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    pub map: bumpalo::collections::Vec<'a, (C::Prop, &'a Bdd<'a, C, T>)>,
    pub open: Openness,
}
impl<'a, C, T> TyAtom for Record<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
}
