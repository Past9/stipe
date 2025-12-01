use crate::ty::{Ty, TyConfig};

pub struct Product<'a, C>(pub &'a Ty<'a, C>, pub &'a Ty<'a, C>)
where
    C: TyConfig;
