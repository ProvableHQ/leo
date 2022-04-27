/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/basic.in
*/

type int = u32;
type number = int;

function main(x: u32, y: bool) -> bool {
    let a: int = x;
    let b: number = 2u32;
    let c: number = 3;
    let d: u32 = a + b + c;
  
	return a == 1u32 && b == 2u32
		&& c == 3u32 && d == 6u32 && y;
}