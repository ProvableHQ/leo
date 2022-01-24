/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(a: [char; 11], y: bool) -> bool {
    let z = a[1..2]; // open ranges are impossible
    return y == (z[0] == 'e');
}
