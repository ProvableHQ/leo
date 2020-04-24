function test() -> (u32, u32) {
    return 4, 4
}

function main() -> (u32, u32) {
    a, b = test()

    return a, b
}

