/*
namespace: Compile
expectation: Pass
input_file: inputs/main_multiple.in
*/

function main(const a: bool, const b: bool, y: bool) -> bool {
    return (a != b) == y;
}
