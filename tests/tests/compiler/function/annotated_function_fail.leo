program test.aleo {
    @noupgrade
    constructor() {}
}

@test
export fn foo(a: u8, b: u8) -> u8 {
    return a + b;
}

@program
export fn bar(a: u8, b: u8) -> u8 {
    return a * b;

    @noupgrade
    constructor() {}
}
