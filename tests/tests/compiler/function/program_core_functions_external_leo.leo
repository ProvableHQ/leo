final fn check_edition::[N: u16]() {
    assert_eq(std::ctx::edition(), N);
}

program child.aleo {
    mapping vals: u32 => u32;

    fn entry(a: u32) -> u32 {
        return a;
    }

    view fn peek(k: u32) -> u32 {
        return vals.get_or_use(k, 0u32);
    }

    @custom
    constructor() {
        assert_neq(
            std::prog::function_checksum::[child.aleo, 'entry'](),
            std::prog::function_checksum::[child.aleo, 'peek']()
        );
        assert_eq(std::prog::checksum::[child.aleo](), std::ctx::checksum());
        check_edition::[0u16]();
        // The computed bound delays this call until the next loop unrolling pass.
        for i in 0u16..(1u16 + 0u16) {
            check_edition::[i]();
        }
    }
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

    @custom
    constructor() {
        assert_neq(
            std::prog::function_checksum::[child.aleo, 'entry'](),
            std::prog::function_checksum::[parent.aleo, 'bar']()
        );
        assert_eq(std::prog::checksum::[parent.aleo](), std::ctx::checksum());
    }
}
