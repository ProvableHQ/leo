
program test.aleo {
    record Token3 {
        owner: address,
        bar: (Bar, Bar),
    }

    struct Bar {
        bar: (Token2, Token2),
    }

    record Token2 {
        owner: address,
        amount: u64,
    }





}
