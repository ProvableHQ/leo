function test(mut a: u32) {
    a = 0;
}

function main() -> u32 {
    let a = 1;
    test(a);

    return a // <- returns 1
}