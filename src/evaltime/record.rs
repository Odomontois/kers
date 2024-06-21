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

type Places = HashMap<Key, usize>;

pub(crate) struct RecordType {
    fields: Vec<(Key, Value)>,
    fields_places: Option<Places>,
}

impl RecordType {
    pub fn field_places(&mut self) -> &Places {
        self.fields_places.get_or_insert_with(|| {
            self.fields
                .iter()
                .enumerate()
                .map(|(i, (k, _))| (k.clone(), i))
                .collect()
        })
    }
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
