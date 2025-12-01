mod arrow;
mod inter;
mod product;
mod record;
mod refr;
mod union;

pub use arrow::Arrow;
pub use product::Product;
pub use record::{Openness, Record};
pub use refr::Ref;

pub trait TyConfig {
    type TyId;
    type Basic;
    type Var;
    type Name;
}

pub enum Ty<'a, C>
where
    C: TyConfig,
{
    Top,
    Bot,
    Var(C::Var),
    Basic(C::Basic),
    Product(Product<'a, C>),
    Arrow(Arrow<'a, C>),
    Record(Record<'a, C>),
    Union(bumpalo::collections::Vec<'a, &'a Self>),
    Inter(bumpalo::collections::Vec<'a, &'a Self>),
    Not(&'a Self),
    Ref(Ref<'a, C>),
}
