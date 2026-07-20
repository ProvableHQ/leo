program child.aleo {
    mapping vals: u32 => u32;

    fn entry(a: u32) -> u32 {
        return a;
    }

    view fn peek(k: u32) -> u32 {
        return vals.get_or_use(k, 0u32);
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    final fn foo() {
        let i: [u8; 32] = std::prog::function_checksum::[child.aleo, 'entry']();
        let j: [u8; 32] = std::prog::function_checksum::[child.aleo, 'peek']();
        assert_neq(i, j);
    }

    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}
