/*
namespace = "Compile"
expectation = "Pass"
*/

// The 'test_dep' program.
program test_dep.aleo {
    mapping Yo: u32 => u32;
    record yeets {
        owner: address,
        val: u32,
    }

    async transition main_dep(a:u32) -> (yeets, Future) {
        let f: Future = finalize_main_dep(a, 1u32);
        let l: yeets = yeets {owner: self.caller, val: 1u32};
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

    async transition main(public a: u32) -> Future {
        return finish(a);
    }

    async function finish(public a: u32) {
        CheatCode::print_mapping(account);
        CheatCode::print_mapping(test_dep.aleo/Yo);
        CheatCode::set_block_height(1u32);
        CheatCode::set_block_height(a);
    }
}
