
// The 'test_dep' program.
program test_dep.aleo {
    mapping Yo: u32 => u32;
    record yeets {
        owner: address,
        val: u32,
    }

    async transition main_dep(a:u32) -> (yeets, Future) {
        let f: Future = finalize_main_dep(a, 1u32);
        let l: yeets = yeets {owner: self.signer, val: 1u32};
        return (l, f);
    }

    async function finalize_main_dep(a:u32, b:u32) {
        Mapping::set(Yo, a, b);
        let c:u32 = a + b;
    }
}

// --- Next Program --- //

import test_dep.aleo;

program test.aleo {
    mapping account: address => u64;

    async transition main(public a: u32, public b: i64) -> Future {
        return finish(a, b);
    }

    async function finish(public a: u32, public b: i64) {
        CheatCode::print_mapping(account);
        CheatCode::print_mapping(test_dep.aleo/Yo);
        CheatCode::set_block_height(1u32);
        CheatCode::set_block_height(a);
        CheatCode::set_block_timestamp(100i64);
        CheatCode::set_block_timestamp(b);
        CheatCode::set_signer("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH");
    }
}
