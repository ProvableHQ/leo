/*
namespace: Compile
expectation: Pass
inputs:
 - tuple.in: |
    [registers]
    r0: (u8, u8) = (0, 0);
    r1: u32 = 100;
*/

// Returns a tuple of tuples.
function tuples() -> ((u8, u8), u32) {
    const a: (u8, u8) = (1, 2);
    const b: u32 = 3;

    return (a, b);
}

function main() {
    const t = tuples();
}
