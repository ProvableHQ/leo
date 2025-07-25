/*
start_height = 20

[case]
program = "test.aleo"
function = "check_starting_height"
input = ["23u32"]
*/

// Note the input height is 23, since the test framework creates a few more blocks to set up the VM.

program test.aleo {
    async transition check_starting_height(height: u32) -> Future {
        return check_starting_height_(height);
    }

    async function check_starting_height_(height: u32) {
        assert(height <= block.height);
    }

    @noupgrade
    async constructor() {}
}
