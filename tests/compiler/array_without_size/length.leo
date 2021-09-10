/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(a: [char; 11], y: bool) -> bool {
    return y == (a.length() == 11);
}
