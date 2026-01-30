struct S {
    a: u32,
    b: u32,
}

fn make_pair(x: u32, y: u32) -> (u32, u32) {
    return (x, y);
}

fn make_triple(x: u32, y: u32, z: u32) -> (u32, u32, u32) {
    return (x, y, z);
}

program tuple_eq_flat_stress.aleo {
    fn test_all() {

        // --- Base tuples
        let t1 = (1u32, 2u32);
        let t2 = (1u32, 3u32);

        // --- 1. assert_eq / assert_neq on 2-tuples
        assert_eq(t1, t1);
        assert_neq(t1, t2);

        // --- 2. assert_eq / assert_neq on 3-tuples
        let a1 = (5u32, 6u32, 7u32);
        let a2 = (5u32, 9u32, 7u32);

        assert_eq(a1, a1);
        assert_neq(a1, a2);

        // --- 3. binary == on 2-tuples
        let eq1 = t1 == t1;
        assert(eq1);

        let eq2 = t1 == t2;
        assert(eq2 == false);

        // --- 4. binary != on 2-tuples
        let ne1 = t1 != t1;
        assert(ne1 == false);

        let ne2 = t1 != t2;
        assert(ne2 == true);

        // --- 5. tuple-returning transition: 2-tuple
        let p1 = make_pair(10u32, 20u32);
        let p2 = make_pair(10u32, 30u32);

        assert_eq(p1, p1);
        assert_neq(p1, p2);

        let c1 = p1 == p1;
        let c2 = p1 == p2;

        assert(c1 == true);
        assert(c2 == false);

        // --- 6. transition returning a flat 3-tuple
        let tr1 = make_triple(1u32, 2u32, 3u32);
        let tr2 = make_triple(1u32, 4u32, 3u32);

        assert_eq(tr1, tr1);
        assert_neq(tr1, tr2);

        let cmp3 = tr1 == tr2;
        assert(cmp3 == false);

        // --- 7. tuple comparison in conditionals
        if ((1u32, 2u32) == (1u32, 2u32)) {
            assert(true);
        } else {
            assert(false);
        }

        if ((1u32, 2u32) != (1u32, 3u32)) {
            assert(true);
        } else {
            assert(false);
        }

        // --- 8. tuple built from struct fields
        let s1 = S { a: 9u32, b: 8u32 };
        let s2 = S { a: 9u32, b: 7u32 };

        assert_eq((s1.a, s1.b), (s1.a, s1.b));
        assert_neq((s1.a, s1.b), (s2.a, s2.b));

        // --- 9. long tuple (4 elements)
        let long1 = (1u32, 2u32, 3u32, 4u32);
        let long2 = (1u32, 2u32, 0u32, 4u32);

        assert_eq(long1, long1);
        assert_neq(long1, long2);

        let long_cmp = long1 == long2;
        assert(long_cmp == false);

        // --- 10. Direct tuple member accesses on tuple expressions
        assert((1u32, 2u32).0 == 1u32);
        assert((1u32, 2u32).1 == 2u32);

        let x = 42u32;
        let y = 99u32;

        assert((x, y).0 == x);
        assert((x, y).1 == y);

        // Larger tuple member access
        let q = (3u32, 4u32, 5u32, 6u32);
        assert(q.2 == 5u32);
        assert(q.3 == 6u32);

        // Member access combined with tuple comparison
        assert(((x, y).0, (x, y).1) == (x, y));
        assert_neq(((x, y).0, (x, y).1), (x, x));   // mismatched tuple

        // --- 11. Ternary selecting between tuples based on tuple comparison
        let tA = (7u32, 8u32);
        let tB = (7u32, 9u32);

        // Condition: tA == tA  → true
        let sel1 = (tA == tA) ? (1u32, 2u32) : (3u32, 4u32);
        assert_eq(sel1, (1u32, 2u32));

        // Condition: tA == tB  → false
        let sel2 = (tA == tB) ? (10u32, 20u32) : (30u32, 40u32);
        assert_eq(sel2, (30u32, 40u32));

        // Condition: comparing tuple expressions directly
        let sel3 = ((1u32, 2u32) != (1u32, 3u32)) ? (5u32, 6u32) : (7u32, 8u32);
        assert_eq(sel3, (5u32, 6u32));

        // Mixed member access + ternary + tuple result
        let a = 11u32;
        let b = 22u32;
        let sel4 = ((a, b).0 == a) ? (a, b) : (0u32, 0u32);
        assert_eq(sel4, (a, b));

    }
}
