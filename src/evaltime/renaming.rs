use crate::Key;
use derive_more::From;

#[derive(Clone, From, Debug, Default)]
pub(crate) enum Renaming {
    #[default]
    Identity,
    Permutation(Vec<usize>),
}

pub enum RenamingError {
    MissingKey(Key),
}
