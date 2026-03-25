program test_invalid_literal_rejection.aleo {
    fn main(public a: field) -> field {
        return a + 1field;
    }

    @noupgrade
    constructor() {}
}
