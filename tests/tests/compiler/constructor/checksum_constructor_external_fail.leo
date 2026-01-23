program dependent.aleo {
    mapping floo: field => [u8; 32];
   
    fn foo() {}
}


// --- Next Program --- //


import dependent.aleo;
program test.aleo {
    fn foo() {}

    @checksum(mapping = "dependent.aleo/bar", key = "1field")
    constructor() {}

    fn food() {
        dependent.aleo/foo();
    }
}
