
program registry.aleo {
    mapping users: address => bool;

    fn register() -> Future {
        return finalize_register(self.caller);
    }
}

onchain fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

// --- Next Program --- //

import registry.aleo;

program relay.aleo {
    fn send(addr: address) -> Future {
        return finalize_send(addr);
    }
}

onchain fn finalize_send(addr: address) {
    let a:bool = Mapping::get(registry.aleo/foo, addr);
    assert_eq(a, true);
}

