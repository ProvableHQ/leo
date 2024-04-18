/*
namespace: Compile
expectation: Pass
*/

program child.aleo {
    record A {
        owner: address,
        val: u32,
    }
    transition mint(owner: address, val: u32) -> A {
        return A {owner: owner, val: val};
    }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    record B {
        owner: address,
        val: u32,
    }
    transition wrapper_mint(owner: address, val: u32) ->  (child.aleo/A, B) {
        return (child.aleo/mint(self.caller, 1u32), B {owner: self.caller, val: val});
    }
}

// --- Next Program --- //

import child.aleo;
import parent.aleo;
program grandparent.aleo {
    record C {
        owner: address,
        val: u32,
    }
    transition double_wrapper_mint(owner: address, val: u32) -> (child.aleo/A, parent.aleo/B, C) {
        let (a,b): (child.aleo/A, parent.aleo/B) = parent.aleo/wrapper_mint(owner,val);
        return (a, b, C {owner: owner, val: val});
    }
}
