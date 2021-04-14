function foo(a: [u8; 1]) {}

function main() {
    const a: [u16; 1] = [1; 1];
    foo(a);
}