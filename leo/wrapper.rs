use leo_compiler::compiler::Compiler;
use leo_compiler::OutputFile;
use snarkvm_curves::bls12_377::Fr;
use snarkvm_eval::edwards_bls12::EdwardsGroupType;
use snarkvm_r1cs::ConstraintSynthesizer;
use snarkvm_r1cs::ConstraintSystem;
use snarkvm_r1cs::SynthesisError;

#[derive(Clone)]
pub struct CompilerWrapper(pub Compiler<'static>, pub leo_ast::Input);

impl<'a> ConstraintSynthesizer<Fr> for CompilerWrapper {
    ///
    /// Synthesizes the circuit with program input.
    ///
    fn generate_constraints<CS: ConstraintSystem<Fr>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let output_directory = self.0.output_directory.clone();
        let package_name = self.0.program_name.clone();

        let result = match self.0.compile::<Fr, EdwardsGroupType, _>(cs, &self.1) {
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(err.exit_code())
            }
            Ok(result) => result,
        };

        // Write results to file
        let output_file = OutputFile::new(&package_name);
        output_file
            .write(&output_directory, result.output.to_string().as_bytes())
            .unwrap();

        Ok(())
    }
}
