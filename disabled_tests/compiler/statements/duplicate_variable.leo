/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
	let x = 1u8;
  let x = true;
	
	return k == true;
}