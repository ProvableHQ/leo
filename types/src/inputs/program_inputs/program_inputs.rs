use crate::{MainInputs, Registers};

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramInputs {
    main: MainInputs,
    registers: Registers,
}

impl ProgramInputs {
    pub fn new() -> Self {
        Self {
            main: MainInputs::new(),
            registers: Registers::new(),
        }
    }
}
