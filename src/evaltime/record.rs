use std::collections::HashMap;

use crate::Key;

use super::values::Value;
use derive_more::From;

#[derive(Clone, From)]
struct Renaming(Vec<usize>);

#[allow(unused)]
pub(crate) struct Record<V> {
    fields: Vec<Value<V>>,
    renaming: Option<Renaming>,
}

type Places = HashMap<Key, usize>;

pub(crate) struct RecordType<V> {
    fields: Vec<(Key, Value<V>)>,
    fields_places: Option<Places>,
}

impl<V> RecordType<V> {
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
fn rename<V>(
    source: &mut RecordType<V>,
    target: &RecordType<V>,
) -> Result<Renaming, RenamingError> {
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
fn test(){
    
}
