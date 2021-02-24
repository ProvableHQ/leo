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

circuit Foo {
    width: u32,
    height: u32,
}

function uno() -> u32 {
    return 1u32
}

function main() {
    // basic test
    console.assert(basic == 8u32);
    // array test
    console.assert(array[0][0] == 0);
    console.assert(array[2][1] == 0);
    // tuple test
    let (x, y) = (1u32, 2u32);
    console.assert(x == 1u32);
    console.assert(y == 2u32);
    // tuple extraction test
    console.assert(a == 1u32);
    console.assert(b == 2u32);
    // group test
    console.assert(simple_group == 1group);
    console.assert(complex_group == (_, 1)group);
    // field test
    console.assert(field_test == 2field);
    // use another const test
    console.assert(use_another_const == 9u32);
    // circuit test
    console.assert(foo.width == 10u32);
    console.assert(foo.height == 20u32);
    // function test
    console.assert(uno == 1u32);
}