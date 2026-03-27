import dep_a.aleo;
import dep_b.aleo;

program main.aleo {
    fn run(x: u32) -> u32 {
        return dep_a.aleo::compute(x) + dep_b.aleo::compute(x);
    }

    @noupgrade
    constructor() {}
}
