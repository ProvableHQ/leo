program test.aleo {
    fn main(a: scalar, b: scalar) -> bool {
        let _c: bool = a > b;
        let _d: bool = a < b;
        let _e: bool = a >= b;
        let f: bool = a <= b;
        return f;
    }

    @noupgrade
    constructor() {}
}
