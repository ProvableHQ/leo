program test.aleo {
    fn main() -> bool {
        return std::ctx::addr();
    }

    fn next() -> bool {
        let a = std::ctx::addr();
        return a;
    }

    @noupgrade
    constructor() {}
}
