function test(a: u32) {
    a = 5;
}

function main() -> u32 {
    let a = 1;
    test(a);

    return a
}