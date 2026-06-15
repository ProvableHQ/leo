program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        let _unused: u32 = x * y;
        let result: u32 = x + y;
        return result;
    }

    @noupgrade
    constructor() {}
}
