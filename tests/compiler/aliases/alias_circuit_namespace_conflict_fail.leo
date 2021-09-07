/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/


type Int = u32;

circuit Int {
	x: u8;
}

function main(y: bool) -> bool {
    return y;
}