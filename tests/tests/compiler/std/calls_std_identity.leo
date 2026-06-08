program main.aleo {
    fn pass_through(x: u32) -> u32 {
        return std::identity_u32(x);
    }

    @noupgrade
    constructor() {}
}
