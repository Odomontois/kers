use std::{collections::HashMap, ops::Add};

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

impl Add<Record> for Record {
    type Output = Record;
    fn add(self, rhs: Record) -> Record {
        let mut data = self.data.clone();
        match (data, &rhs.data) {
            (RecordData::Plain(mut left), RecordData::Plain(right)) => {
                left.extend(right.iter().cloned());
                Record {
                    data: RecordData::Plain(left),
                    renaming: &self.renaming + &rhs.renaming,
                }
            }
        }
    }
}

impl<const N: usize> From<[Value; N]> for Record {
    fn from(data: [Value; N]) -> Self {
        Record {
            data: RecordData::Plain(data.to_vec()),
            renaming: Renaming::Identity,
        }
    }
}
