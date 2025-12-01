mod arrow;
mod product;
mod record;
mod refr;

pub use arrow::Arrow;
pub use product::Product;
pub use record::{Openness, Record};
pub use refr::Ref;

pub trait TyConfig: PartialEq + Eq + PartialOrd + Ord {
    type TyName: PartialEq + Eq + PartialOrd + Ord;
    type Basic: PartialEq + Eq + PartialOrd + Ord;
    type Var: PartialEq + Eq + PartialOrd + Ord;
    type Prop: PartialEq + Eq + PartialOrd + Ord;
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
