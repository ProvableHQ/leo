/*
namespace: Compile
expectation: Pass
*/

program program_a.aleo {
    record X {
        owner: address,
        val: u32,
    }
    transition mint2(input: u32) -> X {
        return X { owner: self.signer, val: input };
    }
}

// --- Next Program --- //

import program_a.aleo;
program program_b.aleo {
    transition foobar(input: program_a.aleo/X) -> u32 {
        return input.val;
    }

    transition boofar(input: program_a.aleo/X) -> program_a.aleo/X {
        return input;
    }
}

// --- Next Program --- //

import program_b.aleo;
program program_c.aleo {
    transition main() {
        assert_eq(1u32, 1u32);
    }
}
