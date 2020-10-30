///
/// Defines a circuit with variable whose type is `Bar`, an undefined circuit.
///
/// Expected output: TypeError
/// Message: "Type circuit `Bar` must be defined before it is used in an expression."
///

circuit Foo {
    b: Bar
}