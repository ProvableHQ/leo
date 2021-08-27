/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/dummy.in
*/

type int = u32;

function return_int_triary() -> (int, int, int) {
	return (1, 2, 3);
}

function return_int_array() -> [int; 3] {
	return [0u32; 3];
}

function main(y: bool) -> bool {
    let a: (int, int, int) = return_int_triary();
    let b: [int; 3] = return_int_array();
  
	return y;
}