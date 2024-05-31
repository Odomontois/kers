use std::collections::BTreeMap;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct FieldAdaptation {
    renamed: usize,
    inner: Adaptation,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct FuncAdaptation {
    domain: Adaptation,
    codomain: Adaptation,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Adaptation {
    Id,
    Record(BTreeMap<usize, FieldAdaptation>),
    Function(Box<FuncAdaptation>),
}

#[test]
fn size_check() {
    use std::mem::size_of;
    assert_eq!(32, size_of::<Adaptation>());
}
