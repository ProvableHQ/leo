/*
<<<<<<< HEAD
start_height = 20
=======
seed = 123456789
min_height = 16
>>>>>>> mainnet

[case]
program = "test.aleo"
function = "check_starting_height"
<<<<<<< HEAD
input = ["23u32"]
=======
input = ["16u32"]
>>>>>>> mainnet
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
