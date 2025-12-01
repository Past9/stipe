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

    pub fn not<T: TyAtom>(&'a self, ty: &'a Bdd<'a, C, T>) -> &'a Bdd<'a, C, T> {
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

#[cfg(test)]
mod tests {
    use crate::{
        Context,
        bdd::{Bdd, TyAtom, Type},
        ty::TyConfig,
    };

    impl TyAtom for String {}

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestName(String);
    impl From<&str> for TestName {
        fn from(value: &str) -> Self {
            TestName(value.to_string())
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBasic(String);
    impl TyAtom for TestBasic {}
    impl From<&str> for TestBasic {
        fn from(value: &str) -> Self {
            TestBasic(value.to_string())
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestVar(String);
    impl TyAtom for TestVar {}
    impl From<&str> for TestVar {
        fn from(value: &str) -> Self {
            TestVar(value.to_string())
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestProp(String);

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestConfig {}
    impl TyConfig for TestConfig {
        type TyName = TestName;
        type Basic = TestBasic;
        type Var = TestVar;
        type Prop = TestProp;
    }

    #[test]
    fn make_types() {
        let ctx: Context<TestConfig> = Context::new();

        let int = ctx.basic("Int".into());
        let boolean = ctx.basic("Boolean".into());

        /*
        println!("not boolean: {:#?}", ctx.not(boolean));

        let union = ctx.inter([int, boolean]);

        println!("union {union:#?}");
        */

        println!("not bool union {:#?}", ctx.inter([int, ctx.not(boolean)]));
        println!("not int union {:#?}", ctx.inter([ctx.not(int), boolean]));
        println!(
            "not both union {:#?}",
            ctx.inter([ctx.not(int), ctx.not(boolean)])
        );
    }

    #[test]
    fn make_types_with_var() {
        let ctx: Context<TestConfig> = Context::new();

        let int = ctx.basic("Int".into());
        let t1 = ctx.var("T1".into());

        let ty_int = Bdd::map_atoms(&ctx.arena, int, &|basic| -> &Type<'_, TestConfig> {
            ctx.arena.alloc(Type::from_basics(&ctx.arena, Bdd::atom))
        });

        /*
        println!("not boolean: {:#?}", ctx.not(boolean));

        let union = ctx.inter([int, boolean]);

        println!("union {union:#?}");
        */

        //println!("union {:#?}", ctx.inter([int, t1]));
    }
}
