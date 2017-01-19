use ir::*;

pub mod environment;
pub mod fulfill;
pub mod infer;
pub mod implemented;
pub mod implemented_with_impl;
pub mod match_clause;
pub mod match_elaborate_clause;
pub mod normalize;
pub mod normalize_application;
pub mod normalize_with_impl;
pub mod goal;
pub mod solver;
pub mod unify;

#[cfg(test)] mod test;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Solution<G> {
    successful: Successful,
    refined_goal: Quantified<Constrained<G>>,
}

impl<G> Solution<G> {
    pub fn map<OP, H>(self, op: OP) -> Solution<H>
        where OP: FnOnce(G) -> H
    {
        Solution {
            successful: self.successful,
            refined_goal: self.refined_goal.map(|c| c.map(op)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Successful {
    Yes,
    Maybe,
}

impl Successful {
    pub fn and(self, s: Successful) -> Successful {
        use self::Successful::*;
        match (self, s) {
            (Yes, Yes) => Yes,
            (Maybe, _) | (_, Maybe) => Maybe,
        }
    }

    pub fn or(self, s: Successful) -> Successful {
        use self::Successful::*;
        match (self, s) {
            (Maybe, Maybe) => Maybe,
            (Yes, _) | (_, Yes) => Yes,
        }
    }
}
