// The source declares `original_prog.aleo`, but `leo deploy --rename` recompiles
// and deploys it on-chain as `renamed_prog.aleo`.
program original_prog.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}
