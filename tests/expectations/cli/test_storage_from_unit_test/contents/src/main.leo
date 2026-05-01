// A program that exercises every supported storage shape and operation, so
// that the accompanying @test functions can read each one back out.

struct Point {
    x: i64,
    y: i64,
}

program storage_demo.aleo {
    // Singletons — one per supported scalar shape, plus a struct and a fixed array.
    storage flag: bool;
    storage counter: u64;
    storage tiny: u8;
    storage signed: i64;
    storage tag: address;
    storage marker: field;
    storage pivot: group;
    storage point: Point;
    storage quad: [u32; 4];

    // Vector — exercised by every permitted operation.
    storage history: [u32];

    // Singleton writers / mutators.
    fn set_flag(b: bool) -> Final {
        return final { flag = b; };
    }

    fn set_counter(v: u64) -> Final {
        return final { counter = v; };
    }

    fn bump_counter() -> Final {
        return final { counter = counter.unwrap_or(0u64) + 1u64; };
    }

    fn clear_counter() -> Final {
        return final { counter = none; };
    }

    fn set_tiny(v: u8) -> Final {
        return final { tiny = v; };
    }

    fn set_signed(v: i64) -> Final {
        return final { signed = v; };
    }

    fn set_tag(a: address) -> Final {
        return final { tag = a; };
    }

    fn set_marker(f: field) -> Final {
        return final { marker = f; };
    }

    fn set_pivot(g: group) -> Final {
        return final { pivot = g; };
    }

    fn set_point(p: Point) -> Final {
        return final { point = p; };
    }

    fn set_quad(a: [u32; 4]) -> Final {
        return final { quad = a; };
    }

    // Vector operations — one transition per primitive op so the @test can
    // drive each independently.
    fn push_history(v: u32) -> Final {
        return final { history.push(v); };
    }

    fn pop_history() -> Final {
        return final {
            let dropped: u32? = history.pop();
            assert(dropped != none);
        };
    }

    fn set_history(i: u32, v: u32) -> Final {
        return final { history.set(i, v); };
    }

    fn swap_remove_history(i: u32) -> Final {
        return final {
            let removed: u32 = history.swap_remove(i);
            assert(removed >= 0u32);
        };
    }

    fn clear_history() -> Final {
        return final { history.clear(); };
    }

    @noupgrade
    constructor() {}
}
