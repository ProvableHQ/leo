
program registry.aleo {
    mapping users: address => bool;

    async fn register() -> Future {
        return finalize_register(self.caller);
    }
    async fn unregister() -> Future {
        return finalize_unregister(self.caller);
    }
}

async fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

async fn finalize_unregister(addr: address) {
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

    async fn send(addr: address, val: u8) -> (message, Future) {
        return (message {owner: addr, data: val}, finalize_send(addr));
    }

    async fn send_without_check(addr: address, val: u8) -> (message, Future) {
        return (message {owner: addr, data: val}, finalize_send_without_check(addr));
    }
}

async fn finalize_send(addr: address) {
    let a:bool = Mapping::get(registry.aleo/users, addr);
    assert_eq(a, true);
}

async fn finalize_send_without_check(addr: address) {
    let a:bool = Mapping::get_or_use(registry.aleo/users, addr, true);
    assert_eq(a, true);
}
