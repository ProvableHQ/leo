program child.aleo {
    struct Point {
        x: field,
        y: field,
    }

    struct Stats {
        values: [u32; 3],
        active: bool,
    }

    storage flag: bool;
    storage scalar_val: scalar;
    storage field_val: field;
    storage group_val: group;

    storage point: Point;
    storage points: [Point; 2];
    storage stats: Stats;
    storage arr_u32: [u32; 3];
    storage arr_bool: [bool; 2];
    storage nested: [[u8; 2]; 2];

    storage counter: u8;
    storage vec: [u8];

    // Initialize everything
    async transition initialize() -> Future {
        return async {
            flag = true;
            scalar_val = 1;
            field_val = 42field;
            group_val = 0group;

            point = Point { x: 1field, y: 2field };
            points = [
                Point { x: 10field, y: 20field },
                Point { x: 30field, y: 40field },
            ];
            stats = Stats { values: [5u32, 10u32, 15u32], active: true };

            arr_u32 = [7u32, 8u32, 9u32];
            arr_bool = [true, false];
            nested = [[1u8, 2u8], [3u8, 4u8]];

            counter = 9u8;
            vec.push(50u8); 
        };
    }

    // Wipes all values to none
    async transition wipe() -> Future {
        return async {
            flag = none;
            scalar_val = none;
            field_val = none;
            group_val = none;

            point = none;
            points = none;
            stats = none;

            arr_u32 = none;
            arr_bool = none;
            nested = none;

            counter = none;
            vec.clear();
        };
    }

    @noupgrade
    async constructor() {}
}


// --- Next Program --- //


import child.aleo;
program test.aleo {
    //
    // ────────────────────────────────────────────────────────────────
    // Case 1: Read initialized external storage
    // ────────────────────────────────────────────────────────────────
    //
    async transition check_initialized() -> Future {
        return async {
            // Primitive unwrapping
            assert(child.aleo/flag.unwrap() == true);
            assert(child.aleo/scalar_val.unwrap() == 1);
            assert(child.aleo/field_val.unwrap() == 42field);
            assert(child.aleo/group_val.unwrap() == 0group);

            // Struct
            let p = child.aleo/point.unwrap();
            assert(p.x == 1field);
            assert(p.y == 2field);

            // Array of structs
            let ps = child.aleo/points.unwrap();
            assert(ps[0].x == 10field);
            assert(ps[1].y == 40field);

            // Struct-with-array
            let s = child.aleo/stats.unwrap();
            assert(s.values[1] == 10u32);
            assert(s.active == true);

            // Arrays
            let a = child.aleo/arr_u32.unwrap();
            assert(a[2] == 9u32);

            let b = child.aleo/arr_bool.unwrap();
            assert(b[0] == true);

            let n = child.aleo/nested.unwrap();
            assert(n[1][1] == 4u8);

            // Additional storage
            assert(child.aleo/counter.unwrap() == 9u8);

            let v2 = child.aleo/vec.get(2);
            assert(v2.unwrap() == 70u8);
        };
    }


    //
    // ────────────────────────────────────────────────────────────────
    // Case 2: After wipe(), everything becomes none
    // ────────────────────────────────────────────────────────────────
    //
    async transition check_wiped() -> Future {
        return async {
            assert(child.aleo/flag == none);
            assert(child.aleo/scalar_val == none);
            assert(child.aleo/field_val == none);
            assert(child.aleo/group_val == none);

            assert(child.aleo/point == none);
            assert(child.aleo/points == none);
            assert(child.aleo/stats == none);

            assert(child.aleo/arr_u32 == none);
            assert(child.aleo/arr_bool == none);
            assert(child.aleo/nested == none);

            assert(child.aleo/counter == none);
            assert(child.aleo/vec.len() == 0);
        };
    }


    //
    // ────────────────────────────────────────────────────────────────
    // Case 3: unwrap_or on wiped storage
    // ────────────────────────────────────────────────────────────────
    //
    async transition check_fallback() -> Future {
        return async {
            assert(child.aleo/flag.unwrap_or(false) == false);
            assert(child.aleo/scalar_val.unwrap_or(0) == 0);
            assert(child.aleo/field_val.unwrap_or(0field) == 0field);
            assert(child.aleo/group_val.unwrap_or(0group) == 0group);

            let p = child.aleo/point.unwrap_or(
                child.aleo/Point { x: 0field, y: 0field }
            );
            assert(p.x == 0field);

            let ps = child.aleo/points.unwrap_or([
                child.aleo/Point { x: 0field, y: 0field },
                child.aleo/Point { x: 0field, y: 0field },
            ]);
            assert(ps[0].x == 0field);

            let s = child.aleo/stats.unwrap_or(
                child.aleo/Stats { values: [0u32, 0u32, 0u32], active: false }
            );
            assert(s.active == false);

            let a = child.aleo/arr_u32.unwrap_or([0u32, 0u32, 0u32]);
            assert(a[1] == 0u32);

            let b = child.aleo/arr_bool.unwrap_or([false, false]);
            assert(b[1] == false);

            let n = child.aleo/nested.unwrap_or([[0u8, 0u8], [0u8, 0u8]]);
            assert(n[0][0] == 0u8);

            assert(child.aleo/counter.unwrap_or(123u8) == 123u8);
        };
    }

    @noupgrade
    async constructor() {}

}
