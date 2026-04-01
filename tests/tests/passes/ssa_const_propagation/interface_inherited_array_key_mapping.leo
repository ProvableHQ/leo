// Test: interface inheritance where both parent and child declare mappings with array key types.
// Exercises that the const propagation pass does not panic when reconstructing inherited
// interface mappings whose key or value types contain array length expressions.

interface Base {
    mapping base_table: [address; 2] => u128;
}

interface Extended: Base {
    mapping ext_table: [bool; 4] => u64;
}

program test.aleo: Extended {
    mapping base_table: [address; 2] => u128;
    mapping ext_table: [bool; 4] => u64;

    fn foo() {}
}
