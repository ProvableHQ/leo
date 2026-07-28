// Negative test for the issue #29589 fix: a program's own exported structs are always emitted, but
// a *library*'s exported struct is inlined into a consumer only on demand. An exported library struct
// that the consumer never uses must NOT be force-kept and inlined, otherwise every consumer would
// carry dead struct definitions. `main.aleo` calls `identity` (which never touches `Unused`), so the
// emitted bytecode must contain `compute`/`identity` but no `struct Unused`.

// --- library: geo_lib --- //

export struct Unused {
    x: u32,
    y: u32,
}

export fn identity(x: u32) -> u32 {
    return x;
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32) -> u32 {
        return geo_lib::identity(a);
    }

    @noupgrade
    constructor() {}
}
