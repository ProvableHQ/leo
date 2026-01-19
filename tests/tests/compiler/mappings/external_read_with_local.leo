
program registry.aleo {
    mapping users: address => bool;

    async fn register() -> Future {
        return finalize_register(self.caller);
    }
}

async fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

// --- Next Program --- //

import registry.aleo;
program relay.aleo {
    mapping users: address => bool;

    async fn send(addr: address) -> Future {
        return finalize_send(addr);
    }
}

async fn finalize_send(addr: address) {
    let a:bool = Mapping::get(relay.aleo/users, addr);
    assert_eq(a, true);
}
