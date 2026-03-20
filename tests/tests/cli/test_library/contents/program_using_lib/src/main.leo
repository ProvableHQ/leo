program program_using_lib.aleo {
    fn main(public a: u32) -> u32 {
        return a + base_lib::BASE_VALUE;
    }

    @noupgrade
    constructor() {}
}
