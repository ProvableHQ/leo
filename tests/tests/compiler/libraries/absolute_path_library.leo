// Test: absolute paths reference imported library items exactly like relative paths,
// both from the root module and from inside a submodule.

// --- library: utils --- //

export struct Wrap {
    val: u32,
}

export fn compute(x: u32) -> u32 {
    return x + 1u32;
}

// --- Next Module: helpers.leo --- //

// A self-qualified absolute path anchors at the library's own root.
export fn double(x: u32) -> u32 {
    return ::utils::compute(x) + ::utils::compute(x);
}

// --- Next Program --- //

program main.aleo {
    fn run(x: u32) -> u32 {
        let relative = utils::compute(x);
        let absolute = ::utils::compute(x);
        let wrapped = ::utils::Wrap { val: absolute };
        return relative + wrapped.val + m::from_module(x);
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: m.leo --- //

// An absolute path inside a module reaches the imported library.
export fn from_module(x: u32) -> u32 {
    return ::utils::helpers::double(x);
}
