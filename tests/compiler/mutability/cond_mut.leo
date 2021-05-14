/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let x = 100i8;
    if false {
        x = 1i8;
        x *= 100i8;
    }
    
    return x == 100i8 == y;
}
