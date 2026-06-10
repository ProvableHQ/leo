program test.aleo {
    fn main(x: field, y: field) -> field {
        let a: field = x + y;
        let b: field = a * 2field;
        return b;
    }

    @noupgrade
    constructor() {}
}
