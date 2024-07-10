use std::{collections::HashMap, ops::Add};

use crate::Key;

use super::{values::Value, Renaming, RenamingError};

type Places = HashMap<Key, usize>;

#[derive(Clone, Debug)]
pub struct RecordType {
    fields: Vec<(Key, Value)>,
    fields_places: Option<Places>,
}

impl Add for RecordType {
    type Output = RecordType;
    fn add(self, rhs: RecordType) -> RecordType {
        let mut fields = self.fields;
        fields.extend(rhs.fields);
        RecordType {
            fields,
            fields_places: None,
        }
    }
}

impl FromIterator<(Key, Value)> for RecordType {
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        RecordType {
            fields: iter.into_iter().collect(),
            fields_places: None,
        }
    }
}

impl<const N: usize> From<[(Key, Value); N]> for RecordType {
    fn from(arr: [(Key, Value); N]) -> Self {
        RecordType {
            fields: arr.to_vec(),
            fields_places: None,
        }
    }
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
