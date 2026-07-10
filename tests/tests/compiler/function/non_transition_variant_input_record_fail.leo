program test.aleo {
    record credits {
        owner: address,
        micrcredits: u64,
    }

    @noupgrade
    constructor() {}
}

export fn foo(a: credits) -> u8 {
    return 1u8;
}

export fn boo(a: address, b: u64) -> credits {
    return credits { owner: a, micrcredits: b };
}
