use leo_compiler::compiler::Compiler;
use leo_compiler::OutputFile;
use snarkvm_algorithms::MerkleParameters;
use snarkvm_curves::bls12_377::Fr;
use snarkvm_dpc::testnet2::Testnet2;
use snarkvm_dpc::{Network, ProgramPublicVariables};
use snarkvm_eval::edwards_bls12::EdwardsGroupType;
use snarkvm_gadgets::{AllocGadget, CRHGadget, UInt8};
use snarkvm_r1cs::ConstraintSynthesizer;
use snarkvm_r1cs::ConstraintSystem;
use snarkvm_r1cs::SynthesisError;

#[derive(Clone)]
pub struct CompilerWrapper<'a>(pub Compiler<'static, 'a>, pub leo_ast::Input);

impl<'a> ConstraintSynthesizer<Fr> for CompilerWrapper<'_> {
    ///
    /// Synthesizes the circuit with program input.
    ///
    fn generate_constraints<CS: ConstraintSystem<Fr>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let output_directory = self.0.output_directory.clone();
        let package_name = self.0.program_name.clone();

        // Alloc program public variables in constraint system.
        let _position = UInt8::alloc_input_vec_le(cs.ns(|| "Alloc position"), &[0u8])?;

        let _transition_id_crh = <Testnet2 as Network>::TransitionIDCRHGadget::alloc_constant(
            &mut cs.ns(|| "Declare the transition ID CRH scheme"),
            || Ok(<Testnet2 as Network>::transition_id_parameters().crh()),
        )?;
        let public_program_variables = ProgramPublicVariables::<Testnet2>::blank();

        let _transition_id =
            <<Testnet2 as Network>::TransitionIDCRHGadget as CRHGadget<_, _>>::OutputGadget::alloc_input(
                cs.ns(|| "Alloc the transition ID"),
                || Ok(public_program_variables.transition_id),
            )?;

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
