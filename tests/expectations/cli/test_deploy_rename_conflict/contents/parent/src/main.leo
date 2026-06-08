import child.aleo;

program parent.aleo {
    fn main(b: u32) -> u32 {
        return child.aleo::main(b);
    }

    @noupgrade
    constructor() {}
}
