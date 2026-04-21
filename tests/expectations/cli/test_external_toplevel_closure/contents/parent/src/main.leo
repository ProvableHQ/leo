import child.aleo;

program parent.aleo {
    @noupgrade
    constructor() {}

    fn main() -> u32 {
        return child.aleo::foo();
    }
}
