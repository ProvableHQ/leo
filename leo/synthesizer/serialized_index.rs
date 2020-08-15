use snarkos_models::gadgets::r1cs::Index;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SerializedIndex {
    Input(usize),
    Aux(usize),
}

impl From<Index> for SerializedIndex {
    fn from(index: Index) -> Self {
        match index {
            Index::Input(idx) => Self::Input(idx),
            Index::Aux(idx) => Self::Aux(idx),
        }
    }
}

impl From<&SerializedIndex> for Index {
    fn from(serialized_index: &SerializedIndex) -> Self {
        match serialized_index {
            SerializedIndex::Input(idx) => Index::Input(idx.clone()),
            SerializedIndex::Aux(idx) => Index::Aux(idx.clone()),
        }
    }
}
