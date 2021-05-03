/*
namespace: Compile
expectation: Pass
inputs:
 - assert.in: |
    [main]
    a: bool = true;

    [registers]
    r0: bool = false;
*/

function main(a: bool) -> bool {
    console.assert(a == true);
    return a == true;
}