import grandchild.aleo;

program child.aleo {
    fn main(b: u32) -> u32 {
        return grandchild.aleo::main(b);
    }

    @noupgrade
    constructor() {}
}
