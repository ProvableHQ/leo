///
/// Defines a circuit `Foo {}`.
/// Attempts to define a second circuit `Foo {}`.
///
/// Expected output: SymbolTableError
/// Message: "Duplicate circuit definition found for `Foo`."
///

circuit Foo {}
circuit Foo {}