use std::collections::HashMap;

use crate::Key;

use super::{values::Value, Renaming, RenamingError};

type Places = HashMap<Key, usize>;

#[derive(Clone, Debug)]
pub struct RecordType {
    fields: Vec<(Key, Value)>,
    fields_places: Option<Places>,
}

impl RecordType {
    pub fn extend(&self, with: RecordType) -> RecordType {
        let mut fields = self.fields.clone();
        fields.extend(with.fields);
        RecordType {
            fields,
            fields_places: None,
        }
    }
    pub fn field_places(&mut self) -> &Places {
        self.fields_places.get_or_insert_with(|| {
            self.fields
                .iter()
                .enumerate()
                .map(|(i, (k, _))| (k.clone(), i))
                .collect()
        })
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
}
