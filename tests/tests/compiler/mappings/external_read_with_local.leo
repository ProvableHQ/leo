
program registry.aleo {
    mapping users: address => bool;

    fn register() -> Final {
        let caller = self.caller;
        return final { finalize_register(caller); };
    }
}

final fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

// --- Next Program --- //

import registry.aleo;
program relay.aleo {
    mapping users: address => bool;

    fn send(addr: address) -> Final {
        return final { finalize_send(addr); };
    }
}

final fn finalize_send(addr: address) {
    let a:bool = Mapping::get(relay.aleo/users, addr);
    assert_eq(a, true);
}
