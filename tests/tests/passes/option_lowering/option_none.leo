program test.aleo {
    @noupgrade
    constructor() {}

    fn main(flag: bool) -> u32 {
        let result: u32? = none;
        return result.unwrap();
    }
}
