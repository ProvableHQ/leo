// FAIL: bare `_function_checksum(...)` calls bypass the `std::prog::function_checksum`
// wrapper — the resolution check has been moved into the intrinsic handler so misuses like
// naming a nonexistent function still get caught.
program test.aleo {
    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}

export final fn foo() {
    let _: [u8; 32] = _function_checksum(test.aleo, 'missing');
}
