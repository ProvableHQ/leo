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

export final fn finalize_register_wrapper(addr: address) {
    finalize_register(addr);
}

// --- Next Program --- //
import registry.aleo;

program relay.aleo {
    fn send(addr: address) -> Final {
        return final {
            registry.aleo::finalize_register(addr);
        };
    }

    fn send_via_wrapper(addr: address) -> Final {
        return final {
            registry.aleo::finalize_register_wrapper(addr);
        };
    }

    @noupgrade
    constructor() {}
}
