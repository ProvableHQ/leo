use super::*;
use indexmap::IndexSet;
use std::hash::Hash;

pub struct SetAppend<T: Hash + Eq + 'static>(IndexSet<T>);

impl<T: Hash + Eq + 'static> Default for SetAppend<T> {
    fn default() -> Self {
        Self(IndexSet::new())
    }
}

impl<T: Hash + Eq + 'static> Monoid for SetAppend<T> {
    fn append(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        SetAppend(self.0)
    }

    fn append_all(mut self, others: impl Iterator<Item=Self>) -> Self {
        let all: Vec<IndexSet<T>> = others.map(|x| x.0).collect();
        let total_size = all.iter().fold(0, |acc, v| acc + v.len());
        self.0.reserve(total_size);
        for item in all.into_iter() {
            self.0.extend(item);
        }
        self
    }
}

impl<T: Hash + Eq + 'static> Into<IndexSet<T>> for SetAppend<T> {
    fn into(self) -> IndexSet<T> {
        self.0
    }
}