use std::ops::Index;

use super::renaming::Renaming;

pub enum Record<V> {
    Plain(Vec<V>),
    Adapted {
        inner: Box<Record<V>>,
        renaming: Renaming,
    },
    Concat {
        left: Box<Record<V>>,
        left_size: usize,
        right: Box<Record<V>>,
    },
}

impl<V> Index<usize> for Record<V> {
    type Output = V;
    fn index(&self, index: usize) -> &V {
        match self {
            Record::Plain(data) => &data[index],
            Record::Adapted { inner, renaming } => &inner[renaming.get(index)],
            Record::Concat {
                left,
                left_size,
                right,
            } => {
                if index < *left_size {
                    &left[index]
                } else {
                    &right[index - *left_size]
                }
            }
        }
    }
}
