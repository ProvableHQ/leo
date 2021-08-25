/*
namespace: Compile
expectation: Pass
input_file:
 - input/dummy.in
*/

function main(y: bool) -> bool {
    let x: [char; _] = ['a', 'b', 'c', 'y'];

    return x[3] == 'y';
}
