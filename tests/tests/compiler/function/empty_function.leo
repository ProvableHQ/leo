
program test.aleo {
    // This should pass and insert
    // a dummy instruction into the empty function.
    function x(a: bool) -> u8 {
        return 0u8;
    }

    transition t(a: bool) -> u8 {
        return x(a);
    }
}
