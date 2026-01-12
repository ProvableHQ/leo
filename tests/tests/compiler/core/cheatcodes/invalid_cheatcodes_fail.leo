
// The 'test_dep' program.
program test_dep.aleo {
    mapping Yo: u32 => u32;
    record yeets {
        owner: address,
        val: u32,
    }

    fn main_dep(a:u32) -> (yeets, Final) {
        let f: Final = final { finalize_main_dep(a, 1u32); };
        let l: yeets = yeets {owner: self.signer, val: 1u32};
        return (l, f);
    }
}

final fn finalize_main_dep(a:u32, b:u32) {
    Mapping::set(Yo, a, b);
    let c:u32 = a + b;
}

// --- Next Program --- //

import test_dep.aleo;

program test.aleo {
    mapping account: address => u64;
    mapping Yo: u32 => u32;

    fn main(public a: u64) -> Final {
        return final { finish(a); };
    }
}

final fn finish(public a: u64, b: address) {
    CheatCode::print_mapping(Yo);
    CheatCode::print_mapping(test_dep.aleo/account);
    CheatCode::set_block_height(1u64);
    CheatCode::set_block_height(a);
    CheatCode::set_block_timestamp(1u32);
    CheatCode::set_block_timestamp(b);
    CheatCode::set_signer(b);
    // Invalid private key.
    CheatCode::set_signer("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPW");
}
