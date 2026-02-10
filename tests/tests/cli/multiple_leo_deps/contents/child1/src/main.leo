import grandchild.aleo;

program child1.aleo {
    record R {
        owner: address,
        f1: field
    }

    fn main(b: u32) -> u32 {
        return grandchild.aleo/main(b);
    }

    @noupgrade
    constructor() {}
}
