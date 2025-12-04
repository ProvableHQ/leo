program tuple_pass_tests.aleo {

    // --------------------------------------------
    // 1. Tuple construction + member access
    // --------------------------------------------
    transition test_members(x: u32, y: u32) -> (u32, u32) {
        let t = (x, y);

        assert_eq(t.0, x);
        assert_eq(t.1, y);

        return (t.0, t.1);
    }

    // --------------------------------------------
    // 2. Tuple equality
    // --------------------------------------------
    transition test_eq(a: u32, b: u32) {
        let t1 = (a, b);
        let t2 = (a, b);

        assert(t1 == t2);
        assert_eq(t1, t2);

        let flag = t1 == t2;
        assert(flag);
    }

    // --------------------------------------------
    // 3. Tuple inequality
    // --------------------------------------------
    transition test_neq(a: u32, b: u32) {
        let t1 = (a, b);
        let t2 = (a + 1u32, b);

        assert(t1 != t2);
        assert_neq(t1, t2);

        let flag = t1 != t2;
        assert(flag);
    }

    // --------------------------------------------
    // 4. Tuple destructuring (pattern binding)
    // --------------------------------------------
    transition test_destructure(a: u32, b: u32) -> u32 {
        let (x, y): (u32, u32) = (a, b);

        assert_eq(x, a);
        assert_eq(y, b);

        return x + y;
    }

    // --------------------------------------------
    // 5. Mixing tuple projections and eq/neq
    // --------------------------------------------
    transition test_mix(a: u32, b: u32) {
        let t = (a, b);

        assert((t.0, t.1) == t);
        assert_neq((t.0 + 1u32, t.1), t);

        let ok = (t.0, t.1) == t;
        assert(ok);
    }

    // --------------------------------------------
    // 6. Ternary operator using tuple comparisons
    // --------------------------------------------
    transition test_ternary(a: u32, b: u32) -> u32 {
        let t1 = (a, b);
        let t2 = (a + 1u32, b);

        // ternary selects different constants based on tuple equality
        let r0 = (t1 == t2) ? t1 : t2;  
        let r1 = (t1 == t1) ? 10u32 : 99u32;
        let r2 = (t1 == t2) ? 77u32 : 11u32;

        assert_eq(r1, 10u32);
        assert_eq(r2, 11u32);

        // ternary with tuple *expressions* directly
        let r3 = ((a, b) != (a, b)) ? 1u32 : 2u32;
        let r4 = ((a, b) != (a + 1u32, b)) ? 3u32 : 4u32;

        assert_eq(r3, 2u32); // false case
        assert_eq(r4, 3u32); // true case

        return r0.0 + r0.1 + r1 + r2 + r3 + r4;
    }
}

