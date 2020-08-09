use crate::{cli::*, cli_types::*, errors::CLIError};
use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};
use leo_package::{
    inputs::*,
    outputs::{ChecksumFile, CircuitFile, OutputsDirectory, OUTPUTS_DIRECTORY_NAME},
    root::Manifest,
    source::{LibFile, MainFile, LIB_FILE_NAME, MAIN_FILE_NAME, SOURCE_DIRECTORY_NAME},
};

use snarkos_algorithms::snark::groth16::KeypairAssembly;
use snarkos_curves::{bls12_377::Bls12_377, edwards_bls12::Fq};
use snarkos_models::{
    curves::PairingEngine,
    gadgets::r1cs::{ConstraintSystem, Index},
};

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct BuildCommand;

impl CLI for BuildCommand {
    type Options = ();
    type Output = Option<(Compiler<Fq, EdwardsGroupType>, bool)>;

    const ABOUT: AboutType = "Compile the current package as a program";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "build";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        let path = current_dir()?;

        // Get the package name
        let manifest = Manifest::try_from(&path)?;
        let package_name = manifest.get_package_name();

        // Sanitize the package path to the root directory
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Construct the path to the output directory
        let mut output_directory = package_path.clone();
        output_directory.push(OUTPUTS_DIRECTORY_NAME);

        // Compile the package starting with the lib.leo file
        if LibFile::exists_at(&package_path) {
            // Construct the path to the library file in the source directory
            let mut lib_file_path = package_path.clone();
            lib_file_path.push(SOURCE_DIRECTORY_NAME);
            lib_file_path.push(LIB_FILE_NAME);

            // Compile the library file but do not output
            let _program = Compiler::<Fq, EdwardsGroupType>::parse_program_without_input(
                package_name.clone(),
                lib_file_path.clone(),
                output_directory.clone(),
            )?;

            log::info!("Compiled library file {:?}", lib_file_path);
        };

        // Compile the main.leo file along with constraints
        if MainFile::exists_at(&package_path) {
            // Create the output directory
            OutputsDirectory::create(&package_path)?;

            // Construct the path to the main file in the source directory
            let mut main_file_path = package_path.clone();
            main_file_path.push(SOURCE_DIRECTORY_NAME);
            main_file_path.push(MAIN_FILE_NAME);

            // Load the input file at `package_name.in`
            let input_string = InputFile::new(&package_name).read_from(&path)?;

            // Load the state file at `package_name.in`
            let state_string = StateFile::new(&package_name).read_from(&path)?;

            // Load the program at `main_file_path`
            let program = Compiler::<Fq, EdwardsGroupType>::parse_program_with_input(
                package_name.clone(),
                main_file_path.clone(),
                output_directory,
                &input_string,
                &state_string,
            )?;

            // Compute the current program checksum
            let program_checksum = program.checksum()?;

            // Generate the program on the constraint system and verify correctness
            {
                use snarkos_utilities::serialize::*;
                fn serialize_circuit<E: PairingEngine, W: Write>(assembly: KeypairAssembly<E>, writer: &mut W) {
                    let fr_size = <<E as PairingEngine>::Fr as ConstantSerializedSize>::SERIALIZED_SIZE;
                    let index_size = <Index as ConstantSerializedSize>::SERIALIZED_SIZE;
                    let tuple_size = fr_size + index_size;
                    println!("tuple size {}", tuple_size);

                    let mut total_size_bytes = 0;

                    let num_inputs = assembly.num_inputs;
                    CanonicalSerialize::serialize(&(num_inputs as u8), writer).unwrap();
                    total_size_bytes += 1;

                    let num_aux = assembly.num_aux;
                    CanonicalSerialize::serialize(&(num_aux as u8), writer).unwrap();
                    total_size_bytes += 1;

                    let num_constraints = assembly.num_constraints;
                    CanonicalSerialize::serialize(&(num_constraints as u8), writer).unwrap();
                    total_size_bytes += 1;

                    // println!("{}", assembly.num_constraints);
                    // serialize each constraint
                    for i in 0..num_constraints {
                        // Serialize the at[i] vector of tuples Vec<(Fr, Index)>

                        let a_len = assembly.at[i].len();
                        CanonicalSerialize::serialize(&(a_len as u8), writer).unwrap();

                        total_size_bytes += 1;
                        total_size_bytes += a_len * tuple_size;

                        for &(ref coeff, index) in assembly.at[i].iter() {
                            // println!("a({:?}, {:?})", coeff, index);
                            CanonicalSerialize::serialize(coeff, writer).unwrap();
                            CanonicalSerialize::serialize(&index, writer).unwrap();
                        }

                        // Serialize the bt[i] vector of tuples Vec<(Fr, Index)>

                        let b_len = assembly.bt[i].len();
                        CanonicalSerialize::serialize(&(b_len as u8), writer).unwrap();

                        total_size_bytes += 1;
                        total_size_bytes += b_len * tuple_size;

                        for &(ref coeff, index) in assembly.bt[i].iter() {
                            // println!("b({:?}, {:?})", coeff, index);
                            CanonicalSerialize::serialize(coeff, writer).unwrap();
                            CanonicalSerialize::serialize(&index, writer).unwrap();
                        }

                        // Serialize the ct[i] vector of tuples Vec<(Fr, Index)>

                        let c_len = assembly.ct[i].len();
                        CanonicalSerialize::serialize(&(c_len as u8), writer).unwrap();

                        total_size_bytes += 1;
                        total_size_bytes += c_len * tuple_size;

                        for &(ref coeff, index) in assembly.ct[i].iter() {
                            // println!("c({:?}, {:?})", coeff, index);
                            CanonicalSerialize::serialize(coeff, writer).unwrap();
                            CanonicalSerialize::serialize(&index, writer).unwrap();
                        }
                    }

                    println!("expected size bytes {:?}", total_size_bytes);
                    // println!("actual size bytes {:?}", writer.len());
                }

                fn deserialize_circuit<E: PairingEngine, R: Read>(reader: &mut R) -> KeypairAssembly<E> {
                    let fr_size = <<E as PairingEngine>::Fr as ConstantSerializedSize>::SERIALIZED_SIZE;
                    let index_size = <Index as ConstantSerializedSize>::SERIALIZED_SIZE;
                    let tuple_size = fr_size + index_size;
                    println!("tuple size {}", tuple_size);

                    let num_inputs = <u8 as CanonicalDeserialize>::deserialize(reader).unwrap() as usize;
                    let num_aux = <u8 as CanonicalDeserialize>::deserialize(reader).unwrap() as usize;
                    let num_constraints = <u8 as CanonicalDeserialize>::deserialize(reader).unwrap() as usize;

                    let mut assembly = KeypairAssembly::<E> {
                        num_inputs,
                        num_aux,
                        num_constraints,
                        at: vec![],
                        bt: vec![],
                        ct: vec![],
                    };

                    for _ in 0..num_constraints {
                        // deserialize each at[i] vector

                        let a_len = <u8 as CanonicalDeserialize>::deserialize(reader).unwrap() as usize;
                        let mut a_lc = vec![];

                        for _ in 0..a_len {
                            let fr = <<E as PairingEngine>::Fr as CanonicalDeserialize>::deserialize(reader).unwrap();
                            let index = <Index as CanonicalDeserialize>::deserialize(reader).unwrap();
                            let tuple = (fr, index);

                            a_lc.push(tuple);
                        }

                        assembly.at.push(a_lc);

                        // deserialize each bt[i] vector

                        let b_len = <u8 as CanonicalDeserialize>::deserialize(reader).unwrap() as usize;
                        let mut b_lc = vec![];

                        for _ in 0..b_len {
                            let fr = <<E as PairingEngine>::Fr as CanonicalDeserialize>::deserialize(reader).unwrap();
                            let index = <Index as CanonicalDeserialize>::deserialize(reader).unwrap();
                            let tuple = (fr, index);

                            b_lc.push(tuple);
                        }

                        assembly.at.push(b_lc);

                        // deserialize each ct[i] vector

                        let c_len = <u8 as CanonicalDeserialize>::deserialize(reader).unwrap() as usize;
                        let mut c_lc = vec![];

                        for _ in 0..c_len {
                            let fr = <<E as PairingEngine>::Fr as CanonicalDeserialize>::deserialize(reader).unwrap();
                            let index = <Index as CanonicalDeserialize>::deserialize(reader).unwrap();
                            let tuple = (fr, index);

                            c_lc.push(tuple);
                        }

                        assembly.at.push(c_lc);
                    }

                    assembly
                }

                let mut cs = KeypairAssembly::<Bls12_377> {
                    num_inputs: 0,
                    num_aux: 0,
                    num_constraints: 0,
                    at: vec![],
                    bt: vec![],
                    ct: vec![],
                };
                let temporary_program = program.clone();
                let output = temporary_program.compile_constraints(&mut cs)?;
                log::debug!("Compiled constraints - {:#?}", output);
                log::debug!("Number of constraints - {:#?}", cs.num_constraints());

                // Serialize circuit
                let mut writer = Vec::new();
                serialize_circuit(cs, &mut writer);
                println!("actual size bytes {:?}", writer.len());

                // Write serialized circuit to circuit `.bytes` file.
                let circuit_file = CircuitFile::new(&package_name);
                circuit_file.write_to(&path, &writer[..])?;

                // Read serialized circuit file
                let serialized = circuit_file.read_from(&package_path)?;
                let same = writer == serialized;
                println!("same {}", same);

                let deserialized = deserialize_circuit::<Bls12_377, _>(&mut &serialized[..]);

                println!("deserialized {:?}", deserialized.num_constraints);

                // println!("{}", std::mem::size_of::<snarkos_curves::bls12_377::Fq>());
                // println!("{}", std::mem::size_of::<snarkos_curves::edwards_bls12::Fq>());
                // println!("{}", <snarkos_models::gadgets::r1cs::Index as ConstantSerializedSize>::SERIALIZED_SIZE);
                // println!("{}", <snarkos_curves::edwards_bls12::Fq as ConstantSerializedSize>::SERIALIZED_SIZE);

                // println!("{}", std::mem::size_of::<snarkos_models::gadgets::r1cs::Index>());
            }

            // If a checksum file exists, check if it differs from the new checksum
            let checksum_file = ChecksumFile::new(&package_name);
            let checksum_differs = if checksum_file.exists_at(&package_path) {
                let previous_checksum = checksum_file.read_from(&package_path)?;
                program_checksum != previous_checksum
            } else {
                // By default, the checksum differs if there is no checksum to compare against
                true
            };

            // If checksum differs, compile the program
            if checksum_differs {
                // Write the new checksum to the output directory
                checksum_file.write_to(&path, program_checksum)?;

                log::debug!("Checksum saved ({:?})", path);
            }

            log::info!("Compiled program file {:?}", main_file_path);

            return Ok(Some((program, checksum_differs)));
        }

        // Return None when compiling a package for publishing
        // The published package does not need to have a main.leo
        Ok(None)
    }
}
