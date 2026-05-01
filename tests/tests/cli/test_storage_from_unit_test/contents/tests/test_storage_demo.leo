// Unit-test program that drives storage_demo.aleo and reads every storage
// variable back via `storage_demo.aleo::<var>` access from inside finalize
// blocks. Each @test gets a fresh deployed ledger, so any state a test
// observes must be produced inside that same test.

import storage_demo.aleo;

program test_storage_demo.aleo {
    // ── Singletons: defaults before any write ────────────────────────────────
    @test
    fn unset_singletons_default() -> Final {
        return final {
            assert(storage_demo.aleo::flag.unwrap_or(false) == false);
            assert(storage_demo.aleo::counter.unwrap_or(0u64) == 0u64);
            assert(storage_demo.aleo::tiny.unwrap_or(0u8) == 0u8);
            assert(storage_demo.aleo::signed.unwrap_or(0i64) == 0i64);
            assert(storage_demo.aleo::marker.unwrap_or(0field) == 0field);
            assert(storage_demo.aleo::quad.unwrap_or([0u32, 0u32, 0u32, 0u32]) == [0u32, 0u32, 0u32, 0u32]);
        };
    }

    // ── Singletons: write-then-read for each supported type ──────────────────
    @test
    fn set_and_read_flag() -> Final {
        let f: Final = storage_demo.aleo::set_flag(true);
        return final {
            f.run();
            assert(storage_demo.aleo::flag.unwrap() == true);
        };
    }

    @test
    fn set_and_read_counter() -> Final {
        let f: Final = storage_demo.aleo::set_counter(42u64);
        return final {
            f.run();
            assert(storage_demo.aleo::counter.unwrap() == 42u64);
            assert(storage_demo.aleo::counter.unwrap_or(0u64) == 42u64);
        };
    }

    @test
    fn bump_counter_from_unset() -> Final {
        let f: Final = storage_demo.aleo::bump_counter();
        return final {
            f.run();
            // unwrap_or(0) inside bump_counter means the first bump yields 1.
            assert(storage_demo.aleo::counter.unwrap() == 1u64);
        };
    }

    @test
    fn bump_counter_twice() -> Final {
        let f1: Final = storage_demo.aleo::bump_counter();
        let f2: Final = storage_demo.aleo::bump_counter();
        return final {
            f1.run();
            f2.run();
            assert(storage_demo.aleo::counter.unwrap() == 2u64);
        };
    }

    @test
    fn clear_counter_resets_default() -> Final {
        let f1: Final = storage_demo.aleo::set_counter(99u64);
        let f2: Final = storage_demo.aleo::clear_counter();
        return final {
            f1.run();
            f2.run();
            assert(storage_demo.aleo::counter.unwrap_or(0u64) == 0u64);
        };
    }

    @test
    fn set_and_read_tiny() -> Final {
        let f: Final = storage_demo.aleo::set_tiny(255u8);
        return final {
            f.run();
            assert(storage_demo.aleo::tiny.unwrap() == 255u8);
        };
    }

    @test
    fn set_and_read_signed() -> Final {
        let f: Final = storage_demo.aleo::set_signed(-1234i64);
        return final {
            f.run();
            assert(storage_demo.aleo::signed.unwrap() == -1234i64);
        };
    }

    @test
    fn set_and_read_tag() -> Final {
        let a: address = aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
        let f: Final = storage_demo.aleo::set_tag(a);
        return final {
            f.run();
            assert(storage_demo.aleo::tag.unwrap() == a);
        };
    }

    @test
    fn set_and_read_marker() -> Final {
        let f: Final = storage_demo.aleo::set_marker(7field);
        return final {
            f.run();
            assert(storage_demo.aleo::marker.unwrap() == 7field);
        };
    }

    @test
    fn set_and_read_pivot() -> Final {
        let g: group = 2group;
        let f: Final = storage_demo.aleo::set_pivot(g);
        return final {
            f.run();
            assert(storage_demo.aleo::pivot.unwrap() == g);
        };
    }

    @test
    fn set_and_read_point() -> Final {
        let p: storage_demo.aleo::Point = storage_demo.aleo::Point { x: -5i64, y: 9i64 };
        let f: Final = storage_demo.aleo::set_point(p);
        return final {
            f.run();
            let observed: storage_demo.aleo::Point = storage_demo.aleo::point.unwrap();
            assert(observed.x == -5i64);
            assert(observed.y == 9i64);
        };
    }

    @test
    fn set_and_read_quad() -> Final {
        let f: Final = storage_demo.aleo::set_quad([1u32, 2u32, 3u32, 4u32]);
        return final {
            f.run();
            let observed: [u32; 4] = storage_demo.aleo::quad.unwrap();
            assert(observed[0u32] == 1u32);
            assert(observed[1u32] == 2u32);
            assert(observed[2u32] == 3u32);
            assert(observed[3u32] == 4u32);
        };
    }

    // ── Vector: empty-state observations ─────────────────────────────────────
    @test
    fn vector_starts_empty() -> Final {
        return final {
            assert(storage_demo.aleo::history.len() == 0u32);
            // Out-of-bounds read returns none; unwrap_or supplies a fallback.
            assert(storage_demo.aleo::history.get(0u32).unwrap_or(0u32) == 0u32);
        };
    }

    // ── Vector: push, len, get ───────────────────────────────────────────────
    @test
    fn push_then_len_and_get() -> Final {
        let f1: Final = storage_demo.aleo::push_history(10u32);
        let f2: Final = storage_demo.aleo::push_history(20u32);
        let f3: Final = storage_demo.aleo::push_history(30u32);
        return final {
            f1.run();
            f2.run();
            f3.run();
            assert(storage_demo.aleo::history.len() == 3u32);
            assert(storage_demo.aleo::history.get(0u32).unwrap() == 10u32);
            assert(storage_demo.aleo::history.get(1u32).unwrap() == 20u32);
            assert(storage_demo.aleo::history.get(2u32).unwrap() == 30u32);
            // Past the end → none.
            assert(storage_demo.aleo::history.get(3u32).unwrap_or(0u32) == 0u32);
        };
    }

    // ── Vector: pop drops the last element ───────────────────────────────────
    @test
    fn pop_drops_last_element() -> Final {
        let f1: Final = storage_demo.aleo::push_history(7u32);
        let f2: Final = storage_demo.aleo::push_history(8u32);
        let f3: Final = storage_demo.aleo::pop_history();
        return final {
            f1.run();
            f2.run();
            f3.run();
            assert(storage_demo.aleo::history.len() == 1u32);
            assert(storage_demo.aleo::history.get(0u32).unwrap() == 7u32);
        };
    }

    // ── Vector: set replaces an existing slot ────────────────────────────────
    @test
    fn set_replaces_slot() -> Final {
        let f1: Final = storage_demo.aleo::push_history(100u32);
        let f2: Final = storage_demo.aleo::push_history(200u32);
        let f3: Final = storage_demo.aleo::set_history(1u32, 999u32);
        return final {
            f1.run();
            f2.run();
            f3.run();
            assert(storage_demo.aleo::history.len() == 2u32);
            assert(storage_demo.aleo::history.get(0u32).unwrap() == 100u32);
            assert(storage_demo.aleo::history.get(1u32).unwrap() == 999u32);
        };
    }

    // ── Vector: swap_remove ──────────────────────────────────────────────────
    @test
    fn swap_remove_pulls_last() -> Final {
        let f1: Final = storage_demo.aleo::push_history(1u32);
        let f2: Final = storage_demo.aleo::push_history(2u32);
        let f3: Final = storage_demo.aleo::push_history(3u32);
        let f4: Final = storage_demo.aleo::push_history(4u32);
        // Removes index 1 (value 2) by swapping with the tail (value 4).
        let f5: Final = storage_demo.aleo::swap_remove_history(1u32);
        return final {
            f1.run();
            f2.run();
            f3.run();
            f4.run();
            f5.run();
            assert(storage_demo.aleo::history.len() == 3u32);
            assert(storage_demo.aleo::history.get(0u32).unwrap() == 1u32);
            assert(storage_demo.aleo::history.get(1u32).unwrap() == 4u32);
            assert(storage_demo.aleo::history.get(2u32).unwrap() == 3u32);
        };
    }

    // ── Vector: clear truncates length to zero ───────────────────────────────
    @test
    fn clear_resets_length() -> Final {
        let f1: Final = storage_demo.aleo::push_history(1u32);
        let f2: Final = storage_demo.aleo::push_history(2u32);
        let f3: Final = storage_demo.aleo::clear_history();
        return final {
            f1.run();
            f2.run();
            f3.run();
            assert(storage_demo.aleo::history.len() == 0u32);
            assert(storage_demo.aleo::history.get(0u32).unwrap_or(0u32) == 0u32);
        };
    }

    // ── Negative tests: unwrapping unset / out-of-bounds storage halts ───────
    @test
    @should_fail
    fn unwrap_unset_singleton_halts() -> Final {
        return final {
            let observed: u64 = storage_demo.aleo::counter.unwrap();
            assert(observed == 0u64);
        };
    }

    @test
    @should_fail
    fn unwrap_oob_get_halts() -> Final {
        return final {
            let observed: u32 = storage_demo.aleo::history.get(0u32).unwrap();
            assert(observed == 0u32);
        };
    }

    @noupgrade
    constructor() {}
}
