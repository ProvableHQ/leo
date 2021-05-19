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

function main(a: u32, b: u32) -> (u32, u32) {
    let (a1, b1) = tuple_conditional();

    return (a + a1, b + b1);
}
