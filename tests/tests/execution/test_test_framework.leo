/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "check_starting_height"
input = ["16u32"]
*/

program test.aleo {
    async transition check_starting_height(height: u32) -> Future {
        return check_starting_height_(height);
    }

    async function check_starting_height_(height: u32) {
        assert(height <= block.height);
    }
}
