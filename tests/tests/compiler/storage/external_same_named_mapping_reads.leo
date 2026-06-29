program left.aleo {
    mapping data: u32 => u8;

    fn noop() -> Final {
        return final {};
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
program right.aleo {
    mapping data: u32 => u8;

    fn noop() -> Final {
        return final {};
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import left.aleo;
import right.aleo;

program caller.aleo {
    mapping out: bool => u8;

    fn read_both(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get_or_use(left.aleo::data, key, 1u8);
            let b: u8 = Mapping::get_or_use(right.aleo::data, key, 2u8);
            Mapping::set(out, false, a + b);
        };
    }

    @noupgrade
    constructor() {}
}
