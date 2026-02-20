program grandchild.aleo {
    record R {
        owner: address,
    }

    fn main(b: u32) -> u32 {
        return b;
    }

    @noupgrade
    constructor() {}
}
