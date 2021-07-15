/*
namespace: Compile
expectation: Pass
inputs:
 - dummy.in: |
    [main]
    y: bool = true;

    [registers]
    r0: bool = false;
*/

function main(y: bool) -> bool {
    const a = 0i8;

    return (-a == 0i8) == y;
}
