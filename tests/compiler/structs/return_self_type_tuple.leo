/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct Circ {
    function fn () -> (Self,u8) {
        return (Self {}, 1);
    }
}

function main (y: bool) -> bool {
	return y;
}