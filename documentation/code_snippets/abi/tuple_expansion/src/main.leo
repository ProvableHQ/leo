program example.aleo {
    fn swap(a: u32, b: u32) -> (u32, u32) {
        return (b, a);
    }

    @noupgrade
    constructor() {}
}
