/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function one() -> bool {
    return true;
}

function main(y: bool) -> bool {
    const a: bool = one() && one();

    return (a == true) == y;
}
