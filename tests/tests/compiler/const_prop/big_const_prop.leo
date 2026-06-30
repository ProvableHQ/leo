export const A: i32 = 1i32 * 3i32;

export const B: i32 = A + 2i32;

export const C: i32 = 0xFFFFFFi32 ^ B;

export const D: i32 = A * 7i32 + C;

program test.aleo {
    fn main() -> i32 {
        const E: i32 = (C ^ D) + A;

        return E;
    }

    @noupgrade
    constructor() {}
}
