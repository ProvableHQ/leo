// Test: a function in a submodule is not visible from a sibling submodule without `export`.
// `parent` (module `[parent]`) tries to call `helpers::secret` (module `[parent, helpers]`).

program test.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: parent.leo --- //

export fn caller() -> u32 {
    return helpers::secret();
}

// --- Next Module: parent/helpers.leo --- //

fn secret() -> u32 {
    return 7u32;
}
