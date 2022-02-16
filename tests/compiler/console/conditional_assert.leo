/*
namespace: Compile
expectation: Pass
inputs:
 - cond.in: |
    [main]
    a: bool = true;
 - cond_2.in: |
    [main]
    a: bool = false;
*/

function main(a: bool) {
    if a {
        console.assert(a == true);
    } else {
        console.assert(a == false);
    }
}
