/*
namespace: Compile
expectation: Pass
input_file: input/main_tuple.in
*/

function main(const x: (u8, bool, u8), y: bool) -> bool {
    return x.0 == 10 && x.1 == true && x.2 == 10;
}
