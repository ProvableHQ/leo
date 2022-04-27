/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/basic.in
*/

type int = u32;

function main(x: u32, y: bool) -> bool {
    let a: int = x;
    let b: int = 2;
    let c: int = a + b;
  
	return a == 1u32 && b == 2u32
		&& c == 3u32 && y;
}