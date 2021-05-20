/*
namespace: Compile
expectation: Pass
inputs: 
 - globals.in: |
    [main]
    a: u32 = 0;

    [registers]
    r0: bool = false;
*/

const basic: u32 = 8;
const array: [u8; (3, 2)] = [[0u8; 2]; 3];
const tuple = (1u32, 2u32);
const (a, b) = (1u32, 2u32);
const simple_group: group = 1group;
const complex_group = (_, 1)group;
const field_test: field = 2;
const use_another_const = basic + 1;
const foo = Foo { width: 10, height: 20 };
const uno = uno();
const character = 'a';
const hello = `Hello, World!`;

circuit Foo {
    width: u32,
    height: u32,
}

function uno() -> u32 {
    return 1u32;
}

function main(a: u32) -> bool {
    let (x, y) = (1u32, 2u32);

    return a == basic
        && basic == 8u32
        && array[0][0] == 0 // arrays
        && array[2][1] == 0
        && x == 1u32 // tuple test
        && y == 2u32 
        && a == 1u32 // tuple extraction test
        && b == 2u32
        && simple_group == 1group // group test
        && complex_group == (_, 1)group
        && field_test == 2field // field test
        && use_another_const == 9u32 // use another const test
        && foo.width == 10u32 // circuit test
        && foo.height == 20u32
        && uno == 1u32 // function test
        && character == 'a' // char test
        && hello == `Hello, World!`;
}
