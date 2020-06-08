use crate::{CircuitMember, Identifier};
use leo_ast::circuits::Circuit as AstCircuit;

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Circuit {
    pub identifier: Identifier,
    pub members: Vec<CircuitMember>,
}

impl<'ast> From<AstCircuit<'ast>> for Circuit {
    fn from(circuit: AstCircuit<'ast>) -> Self {
        let variable = Identifier::from(circuit.identifier);
        let members = circuit
            .members
            .into_iter()
            .map(|member| CircuitMember::from(member))
            .collect();

        Self {
            identifier: variable,
            members,
        }
    }
}

impl Circuit {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circuit {} {{ \n", self.identifier)?;
        for field in self.members.iter() {
            write!(f, "    {}\n", field)?;
        }
        write!(f, "}}")
    }
}

// TODO (Collin): Uncomment when we no longer print out Program
// impl fmt::Display for Circuit {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.format(f)
//     }
// }

impl fmt::Debug for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
