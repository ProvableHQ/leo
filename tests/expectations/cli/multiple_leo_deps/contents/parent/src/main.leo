import child1.aleo;
import child2.aleo;

program parent.aleo {
    fn main(b: u32) -> u32 {
        return child1.aleo/main(b) + child2.aleo/main(b);
    }

    @noupgrade
    constructor() {}
}
