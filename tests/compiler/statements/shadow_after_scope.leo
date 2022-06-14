/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    {
        let x: u8 = 1u8;
    }
    let x: u8 = 1u8;
	return k;
}