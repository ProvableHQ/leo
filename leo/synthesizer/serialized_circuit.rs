use crate::synthesizer::CircuitSynthesizer;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use snarkos_curves::bls12_377::Bls12_377;
use snarkos_errors::curves::FieldError;
use snarkos_models::{
    curves::{Field, Fp256, Fp256Parameters, PairingEngine},
    gadgets::r1cs::{ConstraintSystem, Index},
};
use std::{convert::TryFrom, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct SerializedCircuit {
    pub num_inputs: usize,
    pub num_aux: usize,
    pub num_constraints: usize,

    pub input_assignment: Vec<SerializedField>,
    pub aux_assignment: Vec<SerializedField>,

    pub at: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub bt: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub ct: Vec<Vec<(SerializedField, SerializedIndex)>>,
}

impl SerializedCircuit {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl<E: PairingEngine> From<CircuitSynthesizer<E>> for SerializedCircuit {
    fn from(synthesizer: CircuitSynthesizer<E>) -> Self {
        fn get_serialized_assignments<E: PairingEngine>(assignments: &Vec<E::Fr>) -> Vec<SerializedField> {
            let mut serialized = vec![];

            for assignment in assignments {
                let field = SerializedField::from(assignment);

                serialized.push(field);
            }

            serialized
        }
        fn get_serialized_constraints<E: PairingEngine>(
            constraints: &Vec<(E::Fr, Index)>,
        ) -> Vec<(SerializedField, SerializedIndex)> {
            let mut serialized = vec![];

            for &(ref coeff, index) in constraints {
                let field = SerializedField::from(coeff);
                let index = SerializedIndex::from(index);

                serialized.push((field, index))
            }

            serialized
        }

        let num_inputs = synthesizer.input_assignment.len();
        let num_aux = synthesizer.aux_assignment.len();
        let num_constraints = synthesizer.num_constraints();

        let mut result = Self {
            num_inputs,
            num_aux,
            num_constraints,

            // Assignments
            input_assignment: vec![],
            aux_assignment: vec![],

            // Constraints
            at: vec![],
            bt: vec![],
            ct: vec![],
        };

        // Serialize assignments
        result.input_assignment = get_serialized_assignments::<E>(&synthesizer.input_assignment);
        result.aux_assignment = get_serialized_assignments::<E>(&synthesizer.aux_assignment);

        // Serialize constraints
        for i in 0..num_constraints {
            // Serialize at[i]

            let a_constraints = get_serialized_constraints::<E>(&synthesizer.at[i]);
            result.at.push(a_constraints);

            // Serialize bt[i]

            let b_constraints = get_serialized_constraints::<E>(&synthesizer.bt[i]);
            result.bt.push(b_constraints);

            // Serialize ct[i]

            let c_constraints = get_serialized_constraints::<E>(&synthesizer.ct[i]);
            result.ct.push(c_constraints);
        }

        result
    }
}

impl TryFrom<SerializedCircuit> for CircuitSynthesizer<Bls12_377> {
    type Error = FieldError;

    fn try_from(serialized: SerializedCircuit) -> Result<CircuitSynthesizer<Bls12_377>, Self::Error> {
        fn get_deserialized_assignments(
            assignments: &Vec<SerializedField>,
        ) -> Result<Vec<<Bls12_377 as PairingEngine>::Fr>, FieldError> {
            let mut deserialized = vec![];

            for serialized_assignment in assignments {
                let field = <Bls12_377 as PairingEngine>::Fr::try_from(serialized_assignment)?;

                deserialized.push(field);
            }

            Ok(deserialized)
        }
        fn get_deserialized_constraints(
            constraints: &Vec<(SerializedField, SerializedIndex)>,
        ) -> Result<Vec<(<Bls12_377 as PairingEngine>::Fr, Index)>, FieldError> {
            let mut deserialized = vec![];

            for &(ref serialized_coeff, ref serialized_index) in constraints {
                let field = <Bls12_377 as PairingEngine>::Fr::try_from(serialized_coeff)?;
                let index = Index::from(serialized_index);

                deserialized.push((field, index));
            }

            Ok(deserialized)
        }

        let mut result = CircuitSynthesizer::<Bls12_377> {
            input_assignment: vec![],
            aux_assignment: vec![],
            at: vec![],
            bt: vec![],
            ct: vec![],
        };

        // Deserialize assignments
        result.input_assignment = get_deserialized_assignments(&serialized.input_assignment)?;
        result.aux_assignment = get_deserialized_assignments(&serialized.aux_assignment)?;

        // Deserialize constraints
        for i in 0..serialized.num_constraints {
            // Deserialize at[i]

            let a_constraints = get_deserialized_constraints(&serialized.at[i])?;
            result.at.push(a_constraints);

            // Deserialize bt[i]

            let b_constraints = get_deserialized_constraints(&serialized.bt[i])?;
            result.bt.push(b_constraints);

            // Deserialize ct[i]

            let c_constraints = get_deserialized_constraints(&serialized.ct[i])?;
            result.ct.push(c_constraints);
        }

        Ok(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedField(pub String);

impl<F: Field> From<&F> for SerializedField {
    fn from(field: &F) -> Self {
        // write field to buffer

        let mut buf = Vec::new();

        field.write(&mut buf).unwrap();

        // convert to big integer

        let f_bigint = BigUint::from_bytes_le(&buf);

        let f_string = f_bigint.to_str_radix(10);

        Self(f_string)
    }
}

impl<P: Fp256Parameters> TryFrom<&SerializedField> for Fp256<P> {
    type Error = FieldError;

    fn try_from(serialized: &SerializedField) -> Result<Self, Self::Error> {
        Fp256::<P>::from_str(&serialized.0)
    }
}

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
