export const VALUE: u32 = 10;

program test.aleo {
    fn main() -> u32 {
        const VALUE: u32 = 20;
        return VALUE;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: config.leo --- //

export const admin: u32 = 7u32;

export fn parameter(admin: u32) -> u32 {
    return admin;
}

export fn unused_parameter(admin: u32) -> u32 {
    return 1u32;
}

export fn variable() -> u32 {
    let admin = 9u32;
    return admin;
}

export fn tuple_variable() -> u32 {
    let (admin, other): (u32, u32) = (9u32, 1u32);
    return admin + other;
}

export fn local_constant() -> u32 {
    const admin: u32 = 9u32;
    return admin;
}

export fn iterator() -> u32 {
    for admin: u32 in 0u32..1u32 {}
    return 1u32;
}

export fn generic::[admin: u32]() -> u32 {
    return admin;
}

export struct Sized::[admin: u32] {
    values: [u32; admin],
}

// --- Next Module: config/nested.leo --- //

export const admin: u32 = 11u32;

export fn unused_parameter(admin: u32) -> u32 {
    return 1u32;
}
