program test.aleo {
    record Token {
        // The token owner.
        private owner: address,
        // The token amount.
        amount: u64,
        // The flag.
        constant flag: bool,
    }

    transition mint(owner: address, amount: u64) -> Token {
        return Token {
            owner,
            amount: amount,
            flag: true,
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;

program test_test.aleo {

    @test
    transition mint_1() {
        let result = test.aleo/mint(
            aleo14lskz87tkqwwkyt2z44h64ave5gcwqs6yyfdztus37nupxsj8ypsmqsqcs,
            0u64
        );
        assert_eq(result.owner, aleo14lskz87tkqwwkyt2z44h64ave5gcwqs6yyfdztus37nupxsj8ypsmqsqcs);
        assert_eq(result.amount, 0u64);
        assert_eq(result.flag, true);
    }

    @test
    transition mint_2() {
        let result = test.aleo/mint(
            aleo12keuztkg4cjzxx7hwwmrnnv85dkeqf8pjm877lf6f6lupma2pqrqcl2d8q,
            1u64
        );
        assert_eq(result.owner, aleo12keuztkg4cjzxx7hwwmrnnv85dkeqf8pjm877lf6f6lupma2pqrqcl2d8q);
        assert_eq(result.amount, 1u64);
        assert_eq(result.flag, true);
    }

    @noupgrade
    async constructor() {}
}
