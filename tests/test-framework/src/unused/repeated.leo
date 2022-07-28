/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function one(x: bool) -> bool {
    return x && true;
}

function main(y: bool) -> bool {
    let a: bool = one(y) && one(y);

    return (a == true) == y;
}
