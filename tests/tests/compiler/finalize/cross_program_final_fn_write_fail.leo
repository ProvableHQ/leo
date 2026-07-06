export struct Registration {
    registered: bool,
    count: u32,
}

program registry.aleo {
    mapping users: address => bool;

    fn register(addr: address) -> Final {
        return final { finalize_register(addr); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_register(addr: address) -> Registration {
    Mapping::set(users, addr, true);
    return Registration { registered: true, count: 1u32 };
}

export final fn finalize_register_wrapper(addr: address) -> Registration {
    return Registration { registered: false, ..finalize_register(addr) };
}

export final fn is_registered(addr: address) -> bool {
    return Mapping::get_or_use(users, addr, false);
}

export final fn maybe_register::[WRITE: bool](addr: address) -> bool {
    if WRITE {
        Mapping::set(users, addr, true);
    }
    return is_registered(addr);
}

// --- Next Program --- //
import registry.aleo;

program relay.aleo {
    fn relay(addr: address) -> Final {
        return final {
            assert_eq(registry.aleo::is_registered(addr), true);
            registry.aleo::maybe_register::[false](addr);
            registry.aleo::maybe_register::[true](addr);
            registry.aleo::finalize_register_wrapper(addr);
        };
    }

    @noupgrade
    constructor() {}
}
