/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool, n: bool) -> bool {
    return iter() ? y : n;
}

const function get_true() -> bool {
    return true;
}

function iter() -> bool {
    if get_true() {
        return false;
    } else {
        return true;
    }
}
