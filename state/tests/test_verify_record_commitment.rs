use snarkos_dpc::base_dpc::instantiated::*;

use leo_input::LeoInputParser;
use leo_state::verify_record_commitment;
use leo_typed::Input;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

#[test]
fn test_verify_record_from_file() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    // Generate parameters for the record commitment scheme
    let system_parameters = InstantiatedDPC::generate_system_parameters(&mut rng).unwrap();

    // Load test record state file from `inputs/test.state`
    let file_bytes = include_bytes!("inputs/test.state");
    let file_string = String::from_utf8_lossy(file_bytes);
    let file = LeoInputParser::parse_file(&file_string).unwrap();

    let mut program_input = Input::new();
    program_input.parse_state(file).unwrap();

    let typed_record = program_input.get_record();

    // check record state is correct by verifying commitment
    let _values = verify_record_commitment(typed_record, system_parameters.record_commitment).unwrap();
}
