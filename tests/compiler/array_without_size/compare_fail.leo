/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let x: [u8; _] = [1u8,2];
    let z: bool = x == [1u8,2,3]; // array size mismatch
<<<<<<< HEAD
    return y;
=======
    return y != z;
>>>>>>> 10c208225288e2671f1bc59f8c587c16ea426525
}
