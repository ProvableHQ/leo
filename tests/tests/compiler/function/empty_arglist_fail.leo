@no_inline
export fn x() -> u8 {
    return 0u8;
}

@no_inline
export fn y(a: [u8; 0]) -> [u8; 0] {
    return a;
}

program test.aleo {
    @noupgrade
    constructor() {}
}
