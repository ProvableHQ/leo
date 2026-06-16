program test.aleo {
    fn main(a: group) -> group {
        return 2scalar * a;
    }

    @noupgrade
    constructor() {}
}
