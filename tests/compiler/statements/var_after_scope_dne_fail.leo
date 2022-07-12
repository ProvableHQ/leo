/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    {
        let x: bool = false;
    }
	return k && x;
}