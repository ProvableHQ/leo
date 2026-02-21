// The 'simple' program.
import simple.aleo;

program complex.aleo {
    @noupgrade
    constructor() {}

    fn main(public a: u32, b: u32) -> u32 {
        return simple.aleo/main(a, b);
    }
}
