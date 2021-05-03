/*
namespace: Compile
expectation: Fail
inputs:
 - const.in: |
    [constants]
    a: bool = true;
*/

// Const function input are immutable.
function main(const a: bool) {
    a = false;
}