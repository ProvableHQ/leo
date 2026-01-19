
program program_a.aleo {
    record X {
        owner: address,
        val: u32,
    }
    fn mint2(input: u32) -> X {
        return X { owner: self.signer, val: input };
    }
}

// --- Next Program --- //

import program_a.aleo;
program program_b.aleo {
    fn foobar(input: program_a.aleo/X) -> u32 {
        return input.val;
    }

    fn boofar(input: program_a.aleo/X) -> program_a.aleo/X {
        return input;
    }
}

// --- Next Program --- //

import program_b.aleo;
program program_c.aleo {
    fn main() {
        assert_eq(1u32, 1u32);
    }
}
