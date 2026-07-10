export const MAX: u32 = 100;
export const MAX: u32 = 200;

program test.aleo {
    fn main() -> u32 {
        return MAX;
    }

    @noupgrade
    constructor() {}
}
