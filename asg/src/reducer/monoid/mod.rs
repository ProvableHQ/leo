
mod vec_append;
pub use vec_append::*;

mod set_append;
pub use set_append::*;

mod bool_and;
pub use bool_and::*;

pub trait Monoid: Default {
    fn append(self, other: Self) -> Self;

    fn append_all(self, others: impl Iterator<Item=Self>) -> Self {
        let mut current = self;
        for item in others {
            current = current.append(item);
        }
        current
    }

    fn append_option(self, other: Option<Self>) -> Self {
        match other {
            None => self,
            Some(other) => self.append(other),
        }
    }
}
