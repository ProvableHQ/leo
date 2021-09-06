/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

circuit Int {
	x: u8;
}

type Int = u32;

function main(y: bool) -> bool {
    return y;
}