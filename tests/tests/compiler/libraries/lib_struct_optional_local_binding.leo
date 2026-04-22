// Test: a library struct used as the inner type of an `Optional<T>` local
// binding assigned `none` inside a main-program function body (no storage
// involvement).
//
// Exercises `wrap_none` on a library composite via the type-lowering path
// rather than via storage `.unwrap()`. The zero-synthesis lookup must resolve
// `geo_lib::Point`'s members through `self.composites` — not through
// `storage`-driven reconstruction — confirming that phase-1 registration
// covers library composites referenced purely from function-local types.

// --- library: geo_lib --- //

struct Point {
    x: u32,
    y: u32,
}

// --- Next Program --- //

program user.aleo {
    fn maybe_origin(flag: bool) -> u32 {
        let maybe: geo_lib::Point? = none;
        let fallback: geo_lib::Point = geo_lib::Point { x: 0u32, y: 0u32 };
        let p: geo_lib::Point = maybe.unwrap_or(fallback);
        return p.x;
    }

    @noupgrade
    constructor() {}
}
