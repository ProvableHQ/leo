program dependent.aleo {
    mapping floo: field => [u8; 32];
   
    transition foo() {}
}


// --- Next Program --- //


import dependent.aleo;
program test.aleo {
    transition foo() {}

    @checksum(mapping = "dependent.aleo/bar", key = "1field")
    async constructor() {}

    transition food() {
        dependent.aleo/foo();
    }
}
