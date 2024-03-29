/*
namespace: Compile
expectation: Pass
*/

program registry.aleo {
    mapping users: address => bool;

    transition register() {
        return then finalize(self.caller);
    }
    finalize register(addr: address) {
        Mapping::set(users, addr, true);
    }
    transition unregister() {
        return then finalize(self.caller);
    }
    finalize unregister(addr: address) {
        Mapping::set(users, addr, false);
    }
}

// --- Next Program --- //

import registry.aleo;
program relay.aleo {
    mapping users: address => bool;

    record message {
        owner: address;
        data: u8;
    }

    transition send(addr: address, val: u8) -> message {
        return message {owner: addr, data: val} then finalize(addr);
    }
    finalize send(addr: address) {
        let a:bool = Mapping::get(registry.aleo/users, addr);
        assert_eq(a, true);
    }

    transition send_without_check(addr: address, val: u8) -> message {
        return message {owner: addr, data: val} then finalize(addr);
    }
    finalize send_without_check(addr: address) {
        let a:bool = Mapping::get_or_use(registry.aleo/users, addr, true);
        assert_eq(a, true);
    }
}
