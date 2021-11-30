/*
namespace: Compile
expectation: Pass
input_file: input/length.in
*/

function main(arr1: [u8; 2], const arr2: [u8; 2]) -> bool {
    const len1 = arr1.len();
	const len2 = arr2.len();

	return len1 == len2; 
}