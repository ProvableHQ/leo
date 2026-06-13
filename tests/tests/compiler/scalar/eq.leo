program test.aleo {
    fn main(a: scalar, b: scalar) -> bool {
        let _c: bool = a == b;
        let d: bool = a != b;
        return d;
    }

    @noupgrade
    constructor() {}
}
