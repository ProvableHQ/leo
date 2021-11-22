/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/basic.in
*/

type int = u32;

struct Foo {
	a: u32;
}

struct Bar {
	a: int;
}

function main(x: u32, y: bool) -> bool {
    let a: int = x;
	let f: Foo = Foo { a };
    let b: Bar = Bar { a };
  
	return a == 1u32 && f.a == 1u32
		&& b.a == 1u32 && y;
}