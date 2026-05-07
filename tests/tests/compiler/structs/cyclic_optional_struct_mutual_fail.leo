// Regression for #29405: mutual recursion through `Option` fields.
struct A {
    b: B?,
}

struct B {
    a: A?,
}

program test.aleo {}
