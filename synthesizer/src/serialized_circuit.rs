// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use std::convert::TryFrom;

use eyre::eyre;
use serde::{Deserialize, Serialize};
use snarkvm_curves::{bls12_377::Bls12_377, traits::PairingEngine};
use snarkvm_r1cs::{ConstraintSystem, Index, OptionalVec};

use crate::{CircuitSynthesizer, ConstraintSet, SerializedField, SerializedIndex};
use leo_errors::{LeoError, SnarkVMError};

#[derive(Serialize, Deserialize)]
pub struct SerializedCircuit {
    pub num_public_variables: usize,
    pub num_private_variables: usize,
    pub num_constraints: usize,

    pub public_variables: Vec<SerializedField>,
    pub private_variables: Vec<SerializedField>,

    pub at: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub bt: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub ct: Vec<Vec<(SerializedField, SerializedIndex)>>,
}

impl SerializedCircuit {
    pub fn to_json_string(&self) -> Result<String, LeoError> {
        serde_json::to_string_pretty(&self).map_err(|e| LeoError::from(SnarkVMError::from(eyre!(e))))
    }

    pub fn from_json_string(json: &str) -> Result<Self, LeoError> {
        serde_json::from_str(json).map_err(|e| LeoError::from(SnarkVMError::from(eyre!(e))))
    }
}

impl<E: PairingEngine> From<CircuitSynthesizer<E>> for SerializedCircuit {
    fn from(synthesizer: CircuitSynthesizer<E>) -> Self {
        let num_public_variables = synthesizer.num_public_variables();
        let num_private_variables = synthesizer.num_private_variables();
        let num_constraints = synthesizer.num_constraints();

        // Serialize assignments
        fn get_serialized_assignments<'a, E: PairingEngine, I: Iterator<Item = &'a E::Fr>>(
            assignments: I,
        ) -> Vec<SerializedField> {
            assignments.map(SerializedField::from).collect()
        }

        let public_variables = get_serialized_assignments::<E, _>(synthesizer.public_variables.iter());
        let private_variables = get_serialized_assignments::<E, _>(synthesizer.private_variables.iter());

        // Serialize constraints
        fn get_serialized_constraints<E: PairingEngine>(
            constraints: &[(E::Fr, Index)],
        ) -> Vec<(SerializedField, SerializedIndex)> {
            let mut serialized = Vec::with_capacity(constraints.len());

            for &(ref coeff, index) in constraints {
                let field = SerializedField::from(coeff);
                let index = SerializedIndex::from(index);

                serialized.push((field, index))
            }

            serialized
        }

        let mut at = Vec::with_capacity(num_constraints);
        let mut bt = Vec::with_capacity(num_constraints);
        let mut ct = Vec::with_capacity(num_constraints);

        for i in 0..num_constraints {
            // Serialize at[i]

            let a_constraints = get_serialized_constraints::<E>(&synthesizer.constraints[i].at);
            at.push(a_constraints);

            // Serialize bt[i]

            let b_constraints = get_serialized_constraints::<E>(&synthesizer.constraints[i].bt);
            bt.push(b_constraints);

            // Serialize ct[i]

            let c_constraints = get_serialized_constraints::<E>(&synthesizer.constraints[i].ct);
            ct.push(c_constraints);
        }

        Self {
            num_public_variables,
            num_private_variables,
            num_constraints,
            public_variables,
            private_variables,
            at,
            bt,
            ct,
        }
    }
}

impl TryFrom<SerializedCircuit> for CircuitSynthesizer<Bls12_377> {
    type Error = LeoError;

    fn try_from(serialized: SerializedCircuit) -> Result<CircuitSynthesizer<Bls12_377>, Self::Error> {
        // Deserialize assignments
        fn get_deserialized_assignments(
            assignments: &[SerializedField],
        ) -> Result<OptionalVec<<Bls12_377 as PairingEngine>::Fr>, LeoError> {
            let mut deserialized = OptionalVec::with_capacity(assignments.len());

            for serialized_assignment in assignments {
                let field = <Bls12_377 as PairingEngine>::Fr::try_from(serialized_assignment)
                    .map_err(|e| LeoError::from(SnarkVMError::from(eyre!(e))))?;

                deserialized.insert(field);
            }

            Ok(deserialized)
        }

        let public_variables = get_deserialized_assignments(&serialized.public_variables)?;
        let private_variables = get_deserialized_assignments(&serialized.private_variables)?;

        // Deserialize constraints
        fn get_deserialized_constraints(
            constraints: &[(SerializedField, SerializedIndex)],
        ) -> Result<Vec<(<Bls12_377 as PairingEngine>::Fr, Index)>, LeoError> {
            let mut deserialized = Vec::with_capacity(constraints.len());

            for &(ref serialized_coeff, ref serialized_index) in constraints {
                let field = <Bls12_377 as PairingEngine>::Fr::try_from(serialized_coeff)
                    .map_err(|e| LeoError::from(SnarkVMError::from(eyre!(e))))?;
                let index = Index::from(serialized_index);

                deserialized.push((field, index));
            }

            Ok(deserialized)
        }

        let mut constraints = OptionalVec::with_capacity(serialized.num_constraints);

        for i in 0..serialized.num_constraints {
            // Deserialize at[i]
            let a_constraints = get_deserialized_constraints(&serialized.at[i])?;

            // Deserialize bt[i]
            let b_constraints = get_deserialized_constraints(&serialized.bt[i])?;

            // Deserialize ct[i]
            let c_constraints = get_deserialized_constraints(&serialized.ct[i])?;

            constraints.insert(ConstraintSet {
                at: a_constraints,
                bt: b_constraints,
                ct: c_constraints,
            });
        }

        Ok(CircuitSynthesizer::<Bls12_377> {
            constraints,
            public_variables,
            private_variables,
            namespaces: Default::default(),
        })
    }
}
