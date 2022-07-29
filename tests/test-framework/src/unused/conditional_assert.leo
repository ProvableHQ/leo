/*
namespace: Compile
expectation: Pass
input_file:
    - inputs/true.in
    - inputs/false.in
*/

function main(a: bool) -> bool {
    if a {
        console.assert(a == true);
    } else {
        console.assert(a == false);
    }

    return a;
}