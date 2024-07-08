use std::collections::HashMap;

use crate::Key;

use super::values::Value;
use derive_more::From;

#[derive(Clone, From)]
pub(crate) enum Renaming {
    Identity,
    Permutation(Vec<usize>),
}

pub enum RecordData {
    Plain(Vec<Value>),
}

pub(crate) struct Record {
    data: RecordData,
    renaming: Renaming,
}



pub enum RenamingError {
    MissingKey(Key),
}

#[allow(unused)]
fn rename<V>(source: &mut RecordType, target: &RecordType) -> Result<Renaming, RenamingError> {
    let source = source.field_places();
    target
        .fields
        .iter()
        .map(|(key, _)| {
            source
                .get(key)
                .copied()
                .ok_or_else(|| RenamingError::MissingKey(key.clone()))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into())
}

#[test]
fn test() {}
