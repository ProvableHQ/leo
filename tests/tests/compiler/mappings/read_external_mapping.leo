
program registry.aleo {
    mapping users: address => bool;

    fn register() -> Final {
        return finalize_register(self.caller);
    }
    fn unregister() -> Final {
        return finalize_unregister(self.caller);
    }
}

final fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

final fn finalize_unregister(addr: address) {
    Mapping::set(users, addr, false);
}

// --- Next Program --- //

import registry.aleo;

program relay.aleo {
    mapping users: address => bool;

    record message {
        owner: address,
        data: u8,
    }

    fn send(addr: address, val: u8) -> (message, Final) {
        return (message {owner: addr, data: val}, finalize_send(addr));
    }

    fn send_without_check(addr: address, val: u8) -> (message, Final) {
        return (message {owner: addr, data: val}, finalize_send_without_check(addr));
    }
}

final fn finalize_send(addr: address) {
    let a:bool = Mapping::get(registry.aleo/users, addr);
    assert_eq(a, true);
}

final fn finalize_send_without_check(addr: address) {
    let a:bool = Mapping::get_or_use(registry.aleo/users, addr, true);
    assert_eq(a, true);
}
