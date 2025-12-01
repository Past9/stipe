mod arrow;
mod product;
mod record;
mod refr;

use crate::ty::TyConfig;
use std::{cmp::Ordering, marker::PhantomData};

pub use arrow::Arrow;
pub use product::Product;
pub use record::{Openness, Record};
pub use refr::Refr;

pub trait TyAtom: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug {}

// NOTES: Start with some "whole, top-level" type that includes variables. Only after Step 5
// (original paper, elimination of toplevel variables) do we "separate the constructors"
// (Step 6) to get the more specific BDDs.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type<'a, C>
where
    C: TyConfig,
{
    pub vars: &'a Bdd<'a, C, C::Var>,
    pub basics: &'a Bdd<'a, C, C::Basic>,
    pub products: &'a Bdd<'a, C, Product<'a, C, Type<'a, C>>>,
    pub arrows: &'a Bdd<'a, C, Arrow<'a, C, Type<'a, C>>>,
    pub records: &'a Bdd<'a, C, Record<'a, C, Type<'a, C>>>,
    pub refrs: &'a Bdd<'a, C, Refr<'a, C, Type<'a, C>>>,
    pub _c: PhantomData<C>,
}
impl<'a, C> Type<'a, C>
where
    C: TyConfig,
{
    pub fn empty(arena: &'a bumpalo::Bump) -> Self {
        Self {
            vars: Bdd::bot(arena),
            basics: Bdd::bot(arena),
            products: Bdd::bot(arena),
            arrows: Bdd::bot(arena),
            records: Bdd::bot(arena),
            refrs: Bdd::bot(arena),
            _c: PhantomData,
        }
    }

    pub fn from_basics(arena: &'a bumpalo::Bump, basics: &'a Bdd<'a, C, C::Basic>) -> Self {
        Self {
            vars: Bdd::bot(arena),
            basics,
            products: Bdd::bot(arena),
            arrows: Bdd::bot(arena),
            records: Bdd::bot(arena),
            refrs: Bdd::bot(arena),
            _c: PhantomData,
        }
    }
}
impl<'a, C> TyAtom for Type<'a, C> where C: TyConfig {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bdd<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    Atom {
        //atom: &'a Type<'a, C>,
        atom: &'a T,
        pos: &'a Self,
        lu: &'a Self,
        neg: &'a Self,
        _c: PhantomData<C>,
    },
    Bot,
    Top,
}
impl<'a, C, T> Bdd<'a, C, T>
where
    C: TyConfig,
    T: TyAtom,
{
    pub fn map_atoms<T2: TyAtom, F: Fn(&'a T) -> &'a T2>(
        arena: &'a bumpalo::Bump,
        bdd: &'a Self,
        f: &F,
    ) -> &'a Bdd<'a, C, T2> {
        match bdd {
            Bdd::Atom {
                atom,
                pos,
                lu,
                neg,
                _c,
            } => arena.alloc(Bdd::Atom {
                atom: f(atom),
                pos: Self::map_atoms(arena, pos, f),
                lu: Self::map_atoms(arena, lu, f),
                neg: Self::map_atoms(arena, neg, f),
                _c: PhantomData,
            }),
            Bdd::Bot => Bdd::bot(arena),
            Bdd::Top => Bdd::top(arena),
        }
    }

    pub fn top(arena: &'a bumpalo::Bump) -> &'a Self {
        arena.alloc(Self::Top)
    }

    pub fn bot(arena: &'a bumpalo::Bump) -> &'a Self {
        arena.alloc(Self::Bot)
    }

    pub fn atom(arena: &'a bumpalo::Bump, atom: &'a T) -> &'a Self {
        arena.alloc(Self::Atom {
            atom,
            pos: Self::top(arena),
            lu: Self::bot(arena),
            neg: Self::bot(arena),
            _c: PhantomData,
        })
    }

    pub fn not(arena: &'a bumpalo::Bump, bdd: &'a Self) -> &'a Self {
        match bdd {
            Bdd::Atom {
                atom,
                pos,
                lu,
                neg,
                _c,
            } => arena.alloc(Self::Atom {
                atom,
                pos: Self::not(arena, pos),
                lu: Self::not(arena, lu),
                neg: Self::not(arena, neg),
                _c: PhantomData,
            }),
            Bdd::Bot => Bdd::top(arena),
            Bdd::Top => Bdd::bot(arena),
        }
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
                    _c: _,
                },
                Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: u2,
                    neg: d2,
                    _c: _,
                },
            ) => match a1.cmp(a2) {
                Ordering::Equal => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Self::union(arena, c1, c2),
                    lu: Self::union(arena, u1, u2),
                    neg: Self::union(arena, d1, d2),
                    _c: PhantomData,
                }),
                Ordering::Less => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: c1,
                    lu: Self::union(arena, u1, b2),
                    neg: d1,
                    _c: PhantomData,
                }),
                Ordering::Greater => arena.alloc(Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: Self::union(arena, b1, u2),
                    neg: d2,
                    _c: PhantomData,
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
                    _c: _,
                },
                Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: u2,
                    neg: d2,
                    _c: _,
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
                        _c: PhantomData,
                    }),
                    Ordering::Less => arena.alloc(Self::Atom {
                        atom: a1,
                        pos: Self::inter(arena, c1, b2),
                        lu: Self::inter(arena, u1, b2),
                        neg: Self::inter(arena, d1, b2),
                        _c: PhantomData,
                    }),
                    Ordering::Greater => arena.alloc(Self::Atom {
                        atom: a2,
                        pos: Self::inter(arena, b1, c2),
                        lu: Self::inter(arena, b1, u2),
                        neg: Self::inter(arena, b1, d2),
                        _c: PhantomData,
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
                    _c: _,
                },
            ) => arena.alloc(Self::Atom {
                atom,
                pos: Self::diff(arena, Self::top(arena), b1),
                lu,
                neg: Self::diff(arena, Self::top(arena), b2),
                _c: PhantomData,
            }),

            (
                Self::Atom {
                    atom: a1,
                    pos: c1,
                    lu: u1,
                    neg: d1,
                    _c: _,
                },
                Self::Atom {
                    atom: a2,
                    pos: c2,
                    lu: u2,
                    neg: d2,
                    _c: _,
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
                        _c: PhantomData,
                    }),
                    Ordering::Less => arena.alloc(Self::Atom {
                        atom: a1,
                        pos: Self::diff(arena, Self::union(arena, c1, u1), b2),
                        lu: Self::bot(arena),
                        neg: Self::diff(arena, Self::union(arena, d1, u1), b2),
                        _c: PhantomData,
                    }),
                    Ordering::Greater => arena.alloc(Self::Atom {
                        atom: a2,
                        pos: Self::diff(arena, b1, Self::union(arena, c2, u2)),
                        lu: Self::bot(arena),
                        neg: Self::diff(arena, b1, Self::union(arena, d2, u2)),
                        _c: PhantomData,
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
