use super::*;

pub struct VecAppend<T>(Vec<T>);

impl<T> Default for VecAppend<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T> Monoid for VecAppend<T> {
    fn append(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        VecAppend(self.0)
    }

    fn append_all(mut self, others: impl Iterator<Item=Self>) -> Self {
        let all: Vec<Vec<T>> = others.map(|x| x.0).collect();
        let total_size = all.iter().fold(0, |acc, v| acc + v.len());
        self.0.reserve(total_size);
        for item in all.into_iter() {
            self.0.extend(item);
        }
        self
    }
}

impl<T> Into<Vec<T>> for VecAppend<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}