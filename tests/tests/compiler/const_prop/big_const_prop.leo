const A: i32 = 1i32 * 3i32;

const B: i32 = A + 2i32;

const C: i32 = 0xFFFFFFi32 ^ B;

const D: i32 = A * 7i32 + C;

program test.aleo {
    fn main() -> i32 {
        const E: i32 = (C ^ D) + A;

        return E;
    }
}
