use snarkos_models::curves::{Field, PrimeField};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct GroupType<F: Field + PrimeField> {
    x: F,
    y: F,
}

impl<F: Field + PrimeField> GroupType<F> {
    pub(crate) fn new(x: F, y: F) -> Self {
        Self { x, y }
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<F: Field + PrimeField> fmt::Display for GroupType<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<F: Field + PrimeField> fmt::Debug for GroupType<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
