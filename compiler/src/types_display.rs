//! Format display functions for Leo types.

use crate::{
    Circuit, CircuitMember,
};

use std::fmt;

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitField(ref identifier, ref _type) => {
                write!(f, "{}: {}", identifier, _type)
            }
            CircuitMember::CircuitFunction(ref _static, ref function) => {
                if *_static {
                    write!(f, "static ")?;
                }
                write!(f, "{}", function)
            }
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

// impl fmt::Display for Circuit {// uncomment when we no longer print out Program
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.format(f)
//     }
// }

impl fmt::Debug for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
