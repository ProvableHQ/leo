function foo(a: [u8; 1]) {}

function main() {
    let a: [u16; 1] = [1; 1];
    foo(a);
}