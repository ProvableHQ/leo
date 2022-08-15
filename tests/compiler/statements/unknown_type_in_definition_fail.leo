/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

@program
function main(k: bool) -> bool {
	let b: Foo = 1u8;
    return k == true;
}