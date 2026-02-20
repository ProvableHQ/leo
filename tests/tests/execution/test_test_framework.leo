/*
seed = 987654321
start_height = 20

[case]
program = "test.aleo"
function = "check_starting_height"
input = ["23u32"]
*/

// Note the input height is 23, since the test framework creates a few more blocks to set up the VM.

final fn check_starting_height_(height: u32) {
    assert(height <= block.height);
}

program test.aleo {
    fn check_starting_height(height: u32) -> Final {
        return final { check_starting_height_(height); };
    }

    @noupgrade
    constructor() {}
}
