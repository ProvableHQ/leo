// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use leo_ast::Input;
use leo_input::LeoInputParser;
use leo_state::verify_record_commitment;

use snarkvm_dpc::base_dpc::instantiated::*;

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

// TODO (Collin): Update input to reflect new parameter ordering.
#[test]
#[ignore]
fn test_verify_record_from_file() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    // Generate parameters for the record commitment scheme
    let system_parameters = InstantiatedDPC::generate_system_parameters(&mut rng).unwrap();

    // Load test record state file from `inputs/test.state`
    let file_bytes = include_bytes!("inputs/test_record.state");
    let file_string = String::from_utf8_lossy(file_bytes);
    let file = LeoInputParser::parse_file(&file_string).unwrap();

    let mut program_input = Input::new();
    program_input.parse_state(file).unwrap();

    let typed_record = program_input.get_record();

    // check record state is correct by verifying commitment
    let _values = verify_record_commitment(&system_parameters, typed_record).unwrap();
}
