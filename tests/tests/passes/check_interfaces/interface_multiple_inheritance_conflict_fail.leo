// Test: Interface inheriting conflicting members

export interface A {
    fn process() -> u64;
}

export interface B {
    fn process() -> u32;
}

export interface C: A + B {
}

program test.aleo {
    @noupgrade
    constructor() {}
}
