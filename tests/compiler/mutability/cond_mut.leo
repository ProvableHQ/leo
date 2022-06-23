/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(y: bool) -> bool {
    let x: i128 = 100i128;
    if false {
        x = 1i128;
        x = x * 100i128;
    }
    
    return (x == 100i128) == y;
}
