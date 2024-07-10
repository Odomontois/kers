use std::ops::Add;

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
            _ => todo!("renaming add")
        }
    }
}
