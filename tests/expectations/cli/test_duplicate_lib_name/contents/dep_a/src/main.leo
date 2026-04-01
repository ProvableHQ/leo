program dep_a.aleo {
    fn compute(x: u32) -> u32 {
        return helper::double(x);
    }

    @noupgrade
    constructor() {}
}
