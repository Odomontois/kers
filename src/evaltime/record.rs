use std::collections::HashMap;

use crate::Key;

use super::{values::Value, Renaming};
use derive_more::From;

#[derive(Debug, Clone)]
pub enum RecordData {
    Plain(Vec<Value>),
}

impl Default for RecordData {
    fn default() -> Self {
        RecordData::Plain(Vec::new())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Record {
    data: RecordData,
    renaming: Renaming,
}

impl Record {
    pub(crate) fn extend(&self, with: Record) -> Record {
        let mut data = self.data.clone();
        match (data, with.data) {
            (RecordData::Plain(mut left), RecordData::Plain(right)) => {
                left.extend(right);
                Record {
                    data: RecordData::Plain(left),
                    renaming: self.renaming.clone(),
                }
            }
        }
    }
}
