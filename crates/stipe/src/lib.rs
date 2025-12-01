mod ty;

use crate::ty::{Arrow, Openness, Product, Record, Ref, Ty, TyConfig};
use bumpalo::Bump;
use std::marker::PhantomData;

pub struct Context<C>
where
    C: TyConfig,
{
    arena: Bump,
    _c: PhantomData<C>,
}
impl<'a, C> Context<C>
where
    C: TyConfig,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            arena: Bump::new(),
            _c: PhantomData,
        }
    }

    pub fn top(&'a self) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Top)
    }

    pub fn bot(&'a self) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Bot)
    }

    pub fn var(&'a self, var: C::Var) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Var(var))
    }

    pub fn basic(&'a self, basic: C::Basic) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Basic(basic))
    }

    pub fn product(&'a self, l: &'a Ty<'a, C>, r: &'a Ty<'a, C>) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Product(Product(l, r)))
    }

    pub fn arrow(&'a self, l: &'a Ty<'a, C>, r: &'a Ty<'a, C>) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Arrow(Arrow(l, r)))
    }

    pub fn record<I>(&'a self, open: Openness, props: I) -> &'a Ty<'a, C>
    where
        I: IntoIterator<Item = (C::Name, &'a Ty<'a, C>)>,
    {
        self.arena.alloc(Ty::Record(Record {
            map: bumpalo::collections::Vec::from_iter_in(props, &self.arena),
            open,
        }))
    }

    pub fn union<I>(&'a self, members: I) -> &'a Ty<'a, C>
    where
        I: IntoIterator<Item = &'a Ty<'a, C>>,
    {
        self.arena
            .alloc(Ty::Union(bumpalo::collections::Vec::from_iter_in(
                members,
                &self.arena,
            )))
    }

    pub fn inter<I>(&'a self, members: I) -> &'a Ty<'a, C>
    where
        I: IntoIterator<Item = &'a Ty<'a, C>>,
    {
        self.arena
            .alloc(Ty::Inter(bumpalo::collections::Vec::from_iter_in(
                members,
                &self.arena,
            )))
    }

    pub fn not(&'a self, ty: &'a Ty<'a, C>) -> &'a Ty<'a, C> {
        self.arena.alloc(Ty::Not(ty))
    }

    pub fn refr<I>(&'a self, id: C::TyId, args: I) -> &'a Ty<'a, C>
    where
        I: IntoIterator<Item = &'a Ty<'a, C>>,
    {
        self.arena.alloc(Ty::Ref(Ref {
            id,
            args: bumpalo::collections::Vec::from_iter_in(args, &self.arena),
        }))
    }
}
