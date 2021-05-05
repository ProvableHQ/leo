/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function tuple() -> (bool, bool) {
    return (true, false);
}

function main() -> bool {
    const (a, b) = tuple();

    return a == true && b == false;
}
