use crate::{bdd::Bdd, ty::TyConfig};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Product<'a, C>(pub &'a Bdd<'a, C>, pub &'a Bdd<'a, C>)
where
    C: TyConfig;
