/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/wrong_type_fail.in
*/

type int = u32;

function main(x: u8, y: bool) -> bool {
    let a: int = x;
    
	return a == 1u32 && y;
}