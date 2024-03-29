/*
namespace: Compile
expectation: Fail
*/

program registry.aleo {
    mapping users: address => bool;

    transition register() {
        return then finalize(self.caller);
    }
    finalize register(addr: address) {
        Mapping::set(users, addr, true);
    }
}

// --- Next Program --- //

import registry.aleo;
program relay.aleo {
    transition send(addr: address) {
        return then finalize(addr);
    }
    finalize send(addr: address) {
        let a:bool = Mapping::get(registry.aleo/foo, addr);
        assert_eq(a, true);
    }
}

