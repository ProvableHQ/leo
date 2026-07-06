program registry.aleo {
    mapping users: address => bool;

    fn register() -> Final {
        let caller = self.caller;
        return final { finalize_register(caller); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_register(addr: address) {
    Mapping::set(users, addr, true);
}

export final fn is_registered(addr: address) -> bool {
    return Mapping::get_or_use(users, addr, false);
}

// --- Next Program --- //
import registry.aleo;

program relay.aleo {
    fn send(addr: address) -> Final {
        return final {
            assert_eq(registry.aleo::is_registered(addr), true);
        };
    }

    @noupgrade
    constructor() {}
}
