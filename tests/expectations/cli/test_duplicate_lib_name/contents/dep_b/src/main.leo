program dep_b.aleo {
    fn compute(x: u32) -> u32 {
        return helper::triple(x);
    }

    @noupgrade
    constructor() {}
}
