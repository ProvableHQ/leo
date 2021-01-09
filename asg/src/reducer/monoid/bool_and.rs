use super::*;

pub struct BoolAnd(pub bool);

impl Default for BoolAnd {
    fn default() -> Self {
        BoolAnd(false)
    }
}

impl Monoid for BoolAnd {
    fn append(self, other: Self) -> Self {
        BoolAnd(self.0 && other.0)
    }

    fn append_all(self, others: impl Iterator<Item=Self>) -> Self {
        for item in others {
            if !item.0 {
                return BoolAnd(false)
            }
        }
        BoolAnd(true)
    }
}
