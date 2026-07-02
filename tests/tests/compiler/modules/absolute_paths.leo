struct Merge {
    x: ::c::Shared,
    y: ::c::Gen::[2],
}

program absolute_paths.aleo {
    fn main() -> u32 {
        // Absolute paths from the root module are equivalent to relative ones.
        let direct = ::c::shared(1u32);
        let relative = c::shared(1u32);

        // Both sibling modules reach the shared submodule `c` via absolute paths.
        let from_a = a::use_shared();
        let from_b = b::use_shared();

        let merged = Merge { x: direct, y: ::c::Gen::[2] { vals: [1u32, 2u32] } };
        return direct.val + relative.val + from_a + from_b + merged.y.vals[1u32] + ::c::X;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: a.leo --- //

export fn use_shared() -> u32 {
    let s = ::c::Shared { val: 10u32 };
    return ::c::shared(s.val).val + ::c::X;
}

// --- Next Module: b.leo --- //

fn helper(s: ::c::Shared) -> u32 {
    return s.val;
}

export fn use_shared() -> u32 {
    return helper(::c::shared(20u32));
}

// --- Next Module: c.leo --- //

export const X: u32 = 5;

export struct Shared {
    val: u32,
}

export struct Gen::[N: u32] {
    vals: [u32; N],
}

export fn shared(v: u32) -> Shared {
    return Shared { val: v };
}
