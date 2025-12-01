mod bdd;
mod ty;

use bumpalo::Bump;
use std::marker::PhantomData;
use ty::TyConfig;

use crate::bdd::{Arrow, Bdd, Openness, Product, Record, Refr, TyAtom, Type};

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

    pub fn top(&'a self) -> &'a Bdd<'a, C, Type<'a, C>> {
        Bdd::top(&self.arena)
    }

    pub fn bot(&'a self) -> &'a Bdd<'a, C, Type<C>> {
        Bdd::bot(&self.arena)
    }

    pub fn var(&'a self, var: C::Var) -> &'a Bdd<'a, C, C::Var> {
        Bdd::atom(&self.arena, self.arena.alloc(var))
    }

    pub fn basic(&'a self, basic: C::Basic) -> &'a Bdd<'a, C, C::Basic> {
        Bdd::atom(&self.arena, self.arena.alloc(basic))
    }

    pub fn product(
        &'a self,
        l: &'a Bdd<'a, C, Type<C>>,
        r: &'a Bdd<'a, C, Type<C>>,
    ) -> &'a Bdd<'a, C, Product<C, Type<'a, C>>> {
        Bdd::atom(&self.arena, self.arena.alloc(Product(l, r)))
    }

    pub fn arrow(
        &'a self,
        l: &'a Bdd<'a, C, Type<C>>,
        r: &'a Bdd<'a, C, Type<C>>,
    ) -> &'a Bdd<'a, C, Arrow<C, Type<'a, C>>> {
        Bdd::atom(&self.arena, self.arena.alloc(Arrow(l, r)))
    }

    pub fn record<I>(&'a self, open: Openness, props: I) -> &'a Bdd<'a, C, Record<C, Type<'a, C>>>
    where
        I: IntoIterator<Item = (C::Prop, &'a Bdd<'a, C, Type<'a, C>>)>,
    {
        Bdd::atom(
            &self.arena,
            self.arena.alloc(Record {
                map: bumpalo::collections::Vec::from_iter_in(props, &self.arena),
                open,
            }),
        )
    }

    pub fn not(&'a self, ty: &'a Bdd<'a, C, Type<C>>) -> &'a Bdd<'a, C, Type<C>> {
        Bdd::not(&self.arena, ty)
    }

    pub fn refr<I>(&'a self, id: C::TyName, args: I) -> &'a Bdd<'a, C, Refr<'a, C, Type<'a, C>>>
    where
        I: IntoIterator<Item = &'a Bdd<'a, C, Type<'a, C>>>,
    {
        Bdd::atom(
            &self.arena,
            self.arena.alloc(Refr {
                id,
                args: bumpalo::collections::Vec::from_iter_in(args, &self.arena),
            }),
        )
    }

    pub fn union<I, T: TyAtom>(&'a self, members: I) -> &'a Bdd<'a, C, T>
    where
        I: IntoIterator<Item = &'a Bdd<'a, C, T>>,
    {
        members
            .into_iter()
            .reduce(|acc, ty| Bdd::union(&self.arena, acc, ty))
            .unwrap_or_else(|| Bdd::bot(&self.arena))
    }

    pub fn inter<I, T: TyAtom>(&'a self, members: I) -> &'a Bdd<'a, C, T>
    where
        I: IntoIterator<Item = &'a Bdd<'a, C, T>>,
    {
        members
            .into_iter()
            .reduce(|acc, ty| Bdd::inter(&self.arena, acc, ty))
            .unwrap_or_else(|| Bdd::top(&self.arena))
    }

    /*
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
        I: IntoIterator<Item = (C::Prop, &'a Ty<'a, C>)>,
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

    pub fn refr<I>(&'a self, id: C::TyName, args: I) -> &'a Ty<'a, C>
    where
        I: IntoIterator<Item = &'a Ty<'a, C>>,
    {
        self.arena.alloc(Ty::Ref(Ref {
            id,
            args: bumpalo::collections::Vec::from_iter_in(args, &self.arena),
        }))
    }
    */
}
