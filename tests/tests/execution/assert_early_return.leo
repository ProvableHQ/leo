/*
namespace = "Execute"
expectation = "Pass"
[[cases]]
program = "test.aleo"
function = "main"
input = ["1u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["2u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["3u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["4u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["5u32"]
*/

// The intent is to test that the assertion
// does not trigger if the early return happens.
program test.aleo {
    transition main(x: u32) -> bool {
        if x < 5u32 {
            return true;
        }
        assert(false);
        return true;
    }
}
