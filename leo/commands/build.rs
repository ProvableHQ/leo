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
use snarkos_utilities::serialize::*;

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
                <KeypairAssembly<Bls12_377> as CanonicalSerialize>::serialize(&cs, &mut writer).unwrap();
                // serialize_circuit(cs, &mut writer);
                println!("actual size bytes {:?}", writer.len());

                // Write serialized circuit to circuit `.bytes` file.
                let circuit_file = CircuitFile::new(&package_name);
                circuit_file.write_to(&path, &writer[..])?;

                // Check that we can read the serialized circuit file
                let serialized = circuit_file.read_from(&package_path)?;
                let _deserialized =
                    <KeypairAssembly<Bls12_377> as CanonicalDeserialize>::deserialize(&mut &serialized[..]).unwrap();
                // let _deserialized = deserialize_circuit::<Bls12_377, _>(&mut &serialized[..]);

                // println!("deserialized {:?}", deserialized);
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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializedKeypairAssembly {
    pub num_inputs: usize,
    pub num_aux: usize,
    pub num_constraints: usize,
    pub at: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub bt: Vec<Vec<(SerializedField, SerializedIndex)>>,
    pub ct: Vec<Vec<(SerializedField, SerializedIndex)>>,
}

impl SerializedKeypairAssembly {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl<E: PairingEngine> From<KeypairAssembly<E>> for SerializedKeypairAssembly {
    fn from(assembly: KeypairAssembly<E>) -> Self {
        let mut result = Self {
            num_inputs: assembly.num_inputs,
            num_aux: assembly.num_aux,
            num_constraints: assembly.num_constraints,
            at: vec![],
            bt: vec![],
            ct: vec![],
        };

        for i in 0..assembly.num_constraints {
            let mut a_vec = vec![];
            for &(ref coeff, index) in assembly.at[i].iter() {
                let field = SerializedField::from(coeff);
                let index = SerializedIndex::from(index);

                a_vec.push((field, index))
            }
            result.at.push(a_vec);
        }

        result
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedField(pub Vec<u8>);

impl<E: PairingEngine> From<&<E as PairingEngine>::Fr> for SerializedField {
    fn from(field: &<E as PairingEngine>::Fr) -> Self {
        let mut writer = Vec::new();

        <<E as PairingEngine>::Fr as CanonicalSerialize>::serialize(field, &mut writer).unwrap();

        Self(writer)
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
