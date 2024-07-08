use std::collections::HashMap;

use crate::Key;

use super::values::Value;

type Places = HashMap<Key, usize>;

#[derive(Clone, Debug)]
pub(crate) struct RecordType {
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
}