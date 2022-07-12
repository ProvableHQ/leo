/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function foo() -> bool {
	let b: u8 = 22u8;
	return true;
}

function main(k: bool) -> bool {
	let b: u8 = 11u8;

    return k;
}