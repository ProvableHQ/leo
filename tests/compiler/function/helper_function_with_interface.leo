/*
namespace: Compile
expectation: Pass
*/

@program
function main(public a: u32, b: u32) -> u32 {
    let c: u32 = a + b;
    return c;
}

circuit Board {
   foo: u8,
}

function win(b: Board, p: u8) -> bool {
	return false & true;
}
