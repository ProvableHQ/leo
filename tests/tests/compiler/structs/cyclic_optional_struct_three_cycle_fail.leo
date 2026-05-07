// Regression for #29405: 3-cycle A -> B? -> C? -> A?.
struct A {
    b: B?,
}

struct B {
    c: C?,
}

struct C {
    a: A?,
}

program test.aleo {}
