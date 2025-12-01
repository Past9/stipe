mod product;

use std::cmp::Ordering;

use crate::{bdd::product::Product, ty::TyConfig};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom<'a, C>
where
    C: TyConfig,
{
    Var(C::Var),
    Basic(C::Basic),
    Product(Product<'a, C>),
    //Arrow(Arrow<'a, C>),
    //Record(Record<'a, C>),
    //Ref(Ref<'a, C>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Bdd<'a, C>
where
    C: TyConfig,
{
    Atom {
        atom: &'a Atom<'a, C>,
        pos: &'a Self,
        neg: &'a Self,
    },
    Bot,
    Top,
}
impl<'a, C> Bdd<'a, C>
where
    C: TyConfig,
{
    pub fn top(arena: &'a bumpalo::Bump) -> &'a Self {
        arena.alloc(Self::Top)
    }

    pub fn bot(arena: &'a bumpalo::Bump) -> &'a Self {
        arena.alloc(Self::Bot)
    }

    pub fn union(arena: &'a bumpalo::Bump, b1: &'a Self, b2: &'a Self) -> &'a Self {
        match (b1, b2) {
            (bot @ Bdd::Bot, Bdd::Bot) => bot,
            (top @ Bdd::Top, _) | (_, top @ Bdd::Top) => top,
            (atom @ Bdd::Atom { .. }, Bdd::Bot) | (Bdd::Bot, atom @ Bdd::Atom { .. }) => atom,
            (
                Bdd::Atom {
                    atom: a1,
                    pos: c1,
                    neg: d1,
                },
                Bdd::Atom {
                    atom: a2,
                    pos: c2,
                    neg: d2,
                },
            ) => match a1.cmp(a2) {
                Ordering::Equal => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Bdd::union(arena, c1, c2),
                    neg: Bdd::union(arena, d1, d2),
                }),
                Ordering::Less => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Bdd::union(arena, c1, b2),
                    neg: Bdd::union(arena, d1, b2),
                }),
                Ordering::Greater => arena.alloc(Self::Atom {
                    atom: a2,
                    pos: Bdd::union(arena, b1, c2),
                    neg: Bdd::union(arena, b1, d2),
                }),
            },
        }
    }

    pub fn inter(arena: &'a bumpalo::Bump, b1: &'a Self, b2: &'a Self) -> &'a Self {
        match (b1, b2) {
            (top @ Bdd::Top, Bdd::Top) => top,
            (bot @ Bdd::Bot, _) | (_, bot @ Bdd::Bot) => bot,
            (atom @ Bdd::Atom { .. }, Bdd::Top) | (Bdd::Top, atom @ Bdd::Atom { .. }) => atom,

            (
                Bdd::Atom {
                    atom: a1,
                    pos: c1,
                    neg: d1,
                },
                Bdd::Atom {
                    atom: a2,
                    pos: c2,
                    neg: d2,
                },
            ) => match a1.cmp(a2) {
                Ordering::Equal => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Bdd::inter(arena, c1, c2),
                    neg: Bdd::inter(arena, d1, d2),
                }),
                Ordering::Less => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Bdd::inter(arena, c1, b2),
                    neg: Bdd::inter(arena, d1, b2),
                }),
                Ordering::Greater => arena.alloc(Self::Atom {
                    atom: a2,
                    pos: Bdd::inter(arena, b1, c2),
                    neg: Bdd::inter(arena, b1, d2),
                }),
            },
        }
    }

    pub fn diff(arena: &'a bumpalo::Bump, b1: &'a Self, b2: &'a Self) -> &'a Self {
        match (b1, b2) {
            (_, Bdd::Top) => arena.alloc(Self::Bot),
            (bot @ Bdd::Bot, _) => bot,

            (_, Bdd::Bot) => b1,

            (
                Bdd::Top,
                Bdd::Atom {
                    atom: a,
                    pos: b1,
                    neg: b2,
                },
            ) => arena.alloc(Self::Atom {
                atom: a,
                pos: Bdd::diff(arena, Bdd::top(arena), b1),
                neg: Bdd::diff(arena, Bdd::top(arena), b2),
            }),

            (
                Bdd::Atom {
                    atom: a1,
                    pos: c1,
                    neg: d1,
                },
                Bdd::Atom {
                    atom: a2,
                    pos: c2,
                    neg: d2,
                },
            ) => match a1.cmp(a2) {
                Ordering::Equal => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Bdd::diff(arena, c1, c2),
                    neg: Bdd::diff(arena, d1, d2),
                }),
                Ordering::Less => arena.alloc(Self::Atom {
                    atom: a1,
                    pos: Bdd::diff(arena, c1, b2),
                    neg: Bdd::diff(arena, d1, b2),
                }),
                Ordering::Greater => arena.alloc(Self::Atom {
                    atom: a2,
                    pos: Bdd::diff(arena, b1, c2),
                    neg: Bdd::diff(arena, b1, d2),
                }),
            },
        }
    }
}
