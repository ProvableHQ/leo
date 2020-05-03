function test() -> (u32, u32[2]) {
    return 1, [2, 3]
}

function main() -> (u32[3]) {
    let (a, b) = test();
    // a, u32[2] b = test() <- explicit type also works
    return [a, ...b]
}