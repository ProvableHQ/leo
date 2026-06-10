program test.aleo {
    @noupgrade
    constructor() {}
}

struct A {
    mem: (u8, u16),
}

struct B {
    mems: (A, A),
}
