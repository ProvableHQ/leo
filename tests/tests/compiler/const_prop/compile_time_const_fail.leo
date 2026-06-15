const _A: i32 = 1i32 / 0i32;

const _B: u8 = 255u8 + 1u8;

const _C: field = 0field.inv();

program test.aleo {
    fn main() -> i32 {
        return 0i32;
    }

    @noupgrade
    constructor() {}
}
