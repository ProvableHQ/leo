//
// ────────────────────────────────────────────────────────────────
// Struct and array storage declarations
// ────────────────────────────────────────────────────────────────
//
struct Point {
    x: field,
    y: field,
}

struct Stats {
    values: [u32; 3],
    active: bool,
}

program complex.aleo {
    storage point: Point;
    storage points: [Point; 2];
    storage stats: Stats;
    storage arr_u32: [u32; 3];
    storage arr_bool: [bool; 2];
    storage nested: [[u8; 2]; 2];

    //
    // ────────────────────────────────────────────────────────────────
    // Initialization
    // ────────────────────────────────────────────────────────────────
    //
    fn initialize() -> Final {
        return final {
            // Structs
            point = Point { x: 1field, y: 2field };
            points = [
                Point { x: 10field, y: 20field },
                Point { x: 30field, y: 40field },
            ];

            stats = Stats { values: [5u32, 10u32, 15u32], active: true };

            // Arrays
            arr_u32 = [7u32, 8u32, 9u32];
            arr_bool = [true, false];
            nested = [[1u8, 2u8], [3u8, 4u8]];
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Unset everything 
    // ────────────────────────────────────────────────────────────────
    //
    fn to_none() -> Final {
        return final {
            // Integers
            point = none;
            points = none;
            stats = none;
            arr_u32 = none;
            arr_bool = none;
            nested = none;
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Direct unwrap checks
    // ────────────────────────────────────────────────────────────────
    //
    fn check1() -> Final {
        return final {
            // Struct
            let p = point.unwrap();
            assert(p.x == 1field);
            assert(p.y == 2field);

            // Array of structs
            let ps = points.unwrap();
            assert(ps[0].x == 10field);
            assert(ps[1].y == 40field);

            // Nested structs
            let s = stats.unwrap();
            assert(s.values[1] == 10u32);
            assert(s.active == true);

            // Arrays
            let a = arr_u32.unwrap();
            assert(a[2] == 9u32);

            let b = arr_bool.unwrap();
            assert(b[0] == true);
            assert(b[1] == false);

            let n = nested.unwrap();
            assert(n[0][1] == 2u8);
            assert(n[1][0] == 3u8);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // unwrap_or checks (fallback behavior)
    // ────────────────────────────────────────────────────────────────
    //
    fn check2() -> Final {
        return final {
            // Struct
            let p = point.unwrap_or(Point { x: 0field, y: 0field });
            assert(p.x == 1field);
            assert(p.y == 2field);

            // Array of structs
            let ps = points.unwrap_or([
                Point { x: 0field, y: 0field },
                Point { x: 0field, y: 0field },
            ]);
            assert(ps[0].x == 10field);
            assert(ps[1].y == 40field);

            // Struct with array
            let s = stats.unwrap_or(Stats { values: [0u32, 0u32, 0u32], active: false });
            assert(s.values[2] == 15u32);
            assert(s.active == true);

            // Arrays
            let a = arr_u32.unwrap_or([0u32, 0u32, 0u32]);
            assert(a[1] == 8u32);

            let b = arr_bool.unwrap_or([false, false]);
            assert(b[0] == true);

            let n = nested.unwrap_or([[0u8, 0u8], [0u8, 0u8]]);
            assert(n[1][1] == 4u8);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // No-upgrade constructor
    // ────────────────────────────────────────────────────────────────
    //
    @noupgrade
    constructor() {}
}
