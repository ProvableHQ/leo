program test.aleo {
    @noupgrade
    constructor() {}
}

export struct A {
    mem: (u8, u16),
}

export struct B {
    mems: (A, A),
}
