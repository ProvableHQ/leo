/*
namespace: Compile
expectation: Pass
input_file: input/integers.in
*/

// Returns a tuple using a conditional "if" statement.
function tuple_conditional () -> (
    u32,
    u32
) {
    if true {
        return (1, 1);
    } else {
        return (2, 2);
    }
}

function main() -> (u32, u32) {
    return tuple_conditional();
}
