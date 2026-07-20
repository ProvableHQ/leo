/*
[case]
program = "test.aleo"
function = "read_owner"
input = []
*/

// `self.program_owner` is readable inside a `view fn`: it resolves read-only from the program's
// deploy-time metadata via the finalize register path, just like `block.height`. This mirrors the
// externally-addressed `Program::program_owner(foo.aleo)` form, which was always allowed in views.
// The returned address is the test key's, which the harness sets as the program owner.
program test.aleo {
    fn dummy_transition(x: u32) -> u32 {
        // snarkVM rejects deployments with zero functions, so we keep one minimal transition.
        return x + 1u32;
    }

    view fn read_owner() -> address {
        return self.program_owner;
    }

    @noupgrade
    constructor() {}
}
