// Regression for #29405: mutual recursion through `Option` fields.
export struct A {
    b: B?,
}

export struct B {
    a: A?,
}

program test.aleo {
    @noupgrade
    constructor() {}
}
