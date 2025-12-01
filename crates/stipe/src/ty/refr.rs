use crate::ty::{Ty, TyConfig};

pub struct Ref<'a, C>
where
    C: TyConfig,
{
    pub id: C::TyName,
    pub args: bumpalo::collections::Vec<'a, &'a Ty<'a, C>>,
}
