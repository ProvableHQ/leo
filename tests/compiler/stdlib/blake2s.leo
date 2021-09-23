// namespace: Compile
// expectation: Fail
// input_files:
//  - input/dummy.in

import std.unstable.blake2s.BadCircuit; // `BadCircuit` is not included in the blake2s package

function main() -> bool {
    return false;
}
