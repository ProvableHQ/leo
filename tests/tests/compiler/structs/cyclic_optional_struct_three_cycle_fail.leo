// Regression for #29405: 3-cycle A -> B? -> C? -> A?.
export struct A {
    b: B?,
}

export struct B {
    c: C?,
}

export struct C {
    a: A?,
}

program test.aleo {
    @noupgrade
    constructor() {}
}
