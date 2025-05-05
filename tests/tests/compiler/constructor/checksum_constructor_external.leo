program dependent.aleo {

    mapping floo: field => [u8; 32];
   
    transition foo() {}
}


// --- Next Program --- //

/*
upgrade = { "mode": "checksum", "mapping": "dependent.aleo/floo", "key": "1field" } 
*/

import dependent.aleo;
program test.aleo {

    transition foo() {}

    async constructor() {
        if self.edition > 0u16 {
            let expected: [u8; 32] = Mapping::get(dependent.aleo/floo, 1field);
            assert_eq(self.checksum, expected);
        }
    }

    transition food() {
        dependent.aleo/foo();
    }
}
