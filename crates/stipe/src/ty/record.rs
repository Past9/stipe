use crate::ty::{Ty, TyConfig};

pub struct Record<'a, C>
where
    C: TyConfig,
{
    pub map: bumpalo::collections::Vec<'a, (C::Name, &'a Ty<'a, C>)>,
    pub open: Openness,
}

pub enum Openness {
    Open,
    Closed,
}
