program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        let unused: u32 = x * y;
        let result: u32 = x + y;
        return result;
    }

    @noupgrade
    constructor() {}
}
