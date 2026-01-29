
program registry.aleo {
    mapping users: address => bool;

    fn register() -> Final {
        return finalize_register(self.caller);
    }
}

onchain fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

// --- Next Program --- //

import registry.aleo;
program relay.aleo {
    mapping users: address => bool;

    fn send(addr: address) -> Final {
        return finalize_send(addr);
    }
}

onchain fn finalize_send(addr: address) {
    let a:bool = Mapping::get(relay.aleo/users, addr);
    assert_eq(a, true);
}
