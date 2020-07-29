#[macro_export]
macro_rules! input_section_impl {
    ($($name: ident), *) => ($(

        /// An input section declared in an input file with `[$name]`
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name {
            is_present: bool,
            values: HashMap<String, Option<InputValue>>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    is_present: false,
                    values: HashMap::new(),
                }
            }

            /// Returns an empty version of this struct with `None` values.
            /// Called during constraint synthesis to provide private inputs.
            pub fn empty(&self) -> Self {
                let is_present = self.is_present;
                let mut values = self.values.clone();

                values.iter_mut().for_each(|(_name, value)| {
                    *value = None;
                });

                Self { is_present, values }
            }

            /// Returns `true` if the `$name` variable is passed as input to the main function
            pub fn is_present(&self) -> bool {
                self.is_present
            }

            /// Parses register input definitions and stores them in `self`.
            /// This function is called if the main function input contains the `$name` variable.
            pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
                self.is_present = true;

                for definition in definitions {
                    let name = definition.parameter.variable.value;
                    let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

                    self.values.insert(name, Some(value));
                }

                Ok(())
            }
        }
    )*)
}
