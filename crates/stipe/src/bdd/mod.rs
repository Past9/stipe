mod arrow;
mod product;
mod record;
mod refr;

use std::{cmp::Ordering, marker::PhantomData};

use crate::{
    bdd::{arrow::Arrow, product::Product, record::Record, refr::Refr},
    ty::TyConfig,
};

pub trait TyAtom: PartialEq + Eq + PartialOrd + Ord {}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Type<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    vars: Bdd<'a, C, C::Var>,
    basics: Bdd<'a, C, C::Basic>,
    products: Bdd<'a, C, Product<'a, C, T>>,
    arrows: Bdd<'a, C, Arrow<'a, C, T>>,
    records: Bdd<'a, C, Record<'a, C, T>>,
    refrs: Bdd<'a, C, Refr<'a, C, T>>,
    //_c: PhantomData<C>,
    _t: PhantomData<T>,
}
impl<'a, C, T> TyAtom for Type<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Atom<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    product: Product<'a, C, T>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Bdd<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    Atom {
        atom: &'a Type<'a, C, T>,
        pos: &'a Self,
        lu: &'a Self,
        neg: &'a Self,
    },
    Bot,
    Top,
}
impl<'a, C, T> Bdd<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    pub fn top(arena: &'a bumpalo::Bump) -> &'a Self {
        arena.alloc(Self::Top)
    }

    pub fn bot(arena: &'a bumpalo::Bump) -> &'a Self {
        arena.alloc(Self::Bot)
    }

    pub fn union(arena: &'a bumpalo::Bump, b1: &'a Self, b2: &'a Self) -> &'a Self {
        match (b1, b2) {
            (bot @ Self::Bot, Self::Bot) => bot,
            (top @ Self::Top, _) | (_, top @ Self::Top) => top,
            (atom @ Self::Atom { .. }, Self::Bot) | (Self::Bot, atom @ Self::Atom { .. }) => atom,
            (
                Self::Atom {
                    atom: a1,
                    pos: c1,
                    lu: u1,
                    neg: d1,
                },
                Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: u2,
                    neg: d2,
                },
            ) => match a1.cmp(a2) {
                Ordering::Equal => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Self::union(arena, c1, c2),
                    lu: Self::union(arena, u1, u2),
                    neg: Self::union(arena, d1, d2),
                }),
                Ordering::Less => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: c1,
                    lu: Self::union(arena, u1, b2),
                    neg: d1,
                }),
                Ordering::Greater => arena.alloc(Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: Self::union(arena, b1, u2),
                    neg: d2,
                }),
            },
        }
    }

    pub fn inter(arena: &'a bumpalo::Bump, b1: &'a Self, b2: &'a Self) -> &'a Self {
        match (b1, b2) {
            (top @ Self::Top, Self::Top) => top,
            (bot @ Self::Bot, _) | (_, bot @ Self::Bot) => bot,
            (atom @ Self::Atom { .. }, Self::Top) | (Self::Top, atom @ Self::Atom { .. }) => atom,

            (
                Self::Atom {
                    atom: a1,
                    pos: c1,
                    lu: u1,
                    neg: d1,
                },
                Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: u2,
                    neg: d2,
                },
            ) => Self::simplify_lazy_unions(
                arena,
                match a1.cmp(a2) {
                    Ordering::Equal => arena.alloc(Self::Atom {
                        atom: a1,
                        pos: Self::inter(
                            arena,
                            Self::union(arena, c1, u1),
                            Self::union(arena, c2, u2),
                        ),
                        lu: Self::bot(arena),
                        neg: Self::inter(
                            arena,
                            Self::union(arena, d1, u1),
                            Self::union(arena, d2, u2),
                        ),
                    }),
                    Ordering::Less => arena.alloc(Self::Atom {
                        atom: a1,
                        pos: Self::inter(arena, c1, b2),
                        lu: Self::inter(arena, u1, b2),
                        neg: Self::inter(arena, d1, b2),
                    }),
                    Ordering::Greater => arena.alloc(Self::Atom {
                        atom: a2,
                        pos: Self::inter(arena, b1, c2),
                        lu: Self::inter(arena, b1, u2),
                        neg: Self::inter(arena, b1, d2),
                    }),
                },
            ),
        }
    }

    pub fn diff(arena: &'a bumpalo::Bump, b1: &'a Self, b2: &'a Self) -> &'a Self {
        match (b1, b2) {
            (_, Self::Top) => arena.alloc(Self::Bot),
            (bot @ Self::Bot, _) => bot,

            (_, Self::Bot) => b1,

            (
                Self::Top,
                Self::Atom {
                    atom,
                    pos: b1,
                    lu,
                    neg: b2,
                },
            ) => arena.alloc(Self::Atom {
                atom,
                pos: Self::diff(arena, Self::top(arena), b1),
                lu,
                neg: Self::diff(arena, Self::top(arena), b2),
            }),

            (
                Self::Atom {
                    atom: a1,
                    pos: c1,
                    lu: u1,
                    neg: d1,
                },
                Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: u2,
                    neg: d2,
                },
            ) => Self::simplify_lazy_unions(
                arena,
                match a1.cmp(a2) {
                    Ordering::Equal => arena.alloc(Self::Atom {
                        atom: a1,
                        pos: Self::diff(
                            arena,
                            Self::union(arena, c1, u1),
                            Self::union(arena, c2, u2),
                        ),
                        lu: Self::bot(arena),
                        neg: Self::diff(
                            arena,
                            Self::union(arena, d1, u1),
                            Self::union(arena, d2, u2),
                        ),
                    }),
                    Ordering::Less => arena.alloc(Self::Atom {
                        atom: a1,
                        pos: Self::diff(arena, Self::union(arena, c1, u1), b2),
                        lu: Self::bot(arena),
                        neg: Self::diff(arena, Self::union(arena, d1, u1), b2),
                    }),
                    Ordering::Greater => arena.alloc(Self::Atom {
                        atom: a2,
                        pos: Self::diff(arena, b1, Self::union(arena, c2, u2)),
                        lu: Self::bot(arena),
                        neg: Self::diff(arena, b1, Self::union(arena, d2, u2)),
                    }),
                },
            ),
        }
    }

    fn simplify_lazy_unions(arena: &'a bumpalo::Bump, bdd: &'a Self) -> &'a Self {
        match bdd {
            Self::Atom { pos, lu, neg, .. } if pos != neg && **lu == Self::Top => lu,
            Self::Atom { pos, lu, neg, .. } if pos == neg => Self::union(arena, pos, lu),
            _ => bdd,
        }
    }
}
