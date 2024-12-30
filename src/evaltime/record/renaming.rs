use std::ops::{Add, Index};

use crate::Key;
use derive_more::From;

#[derive(Clone, From, Debug, Default)]
pub(crate) enum Renaming {
    #[default]
    Identity,
    Permutation(Vec<usize>),
}

use Renaming::*;

pub enum RenamingError {
    MissingKey(Key),
}

impl Add<&Renaming> for &Renaming {
    type Output = Renaming;
    fn add(self, rhs: &Renaming) -> Renaming {
        match (self, rhs) {
            (Identity, Identity) => Identity,
            _ => todo!("renaming add"),
        }
    }
}

impl Renaming {
    pub fn get(&self, index: usize) -> usize {
        match self {
            Identity => index,
            Permutation(perm) => perm.get(index).copied().unwrap_or(0),
        }
    }
}
