/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    return y == confirm();
}

const function confirm() -> bool {
    return true;
}
