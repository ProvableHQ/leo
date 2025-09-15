/*
seed = 1122334455
min_height = 16

[case]
program = "none_cases.aleo"
function = "integers_none_behavior"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "none_cases.aleo"
function = "primitives_none_behavior"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "none_cases.aleo"
function = "arrays_none_behavior"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "none_cases.aleo"
function = "structs_none_behavior"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program none_cases.aleo {
    //
    // ────────────────────────────────────────────────────────────────
    // Storage declarations (all non-optional)
    // ────────────────────────────────────────────────────────────────
    //
    storage counter_u32: u32;
    storage flag: bool;
    storage scalar_val: scalar;
    storage field_val: field;
    storage points: [Point; 2];
    storage container: Container;

    struct Point {
        x: field,
        y: field,
    }

    struct Container {
        id: u8,
        point: Point, // non-optional now
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Integers none behavior
    // ────────────────────────────────────────────────────────────────
    //
    async transition integers_none_behavior() -> Future {
        return async {
            assert(counter_u32 == none);
            assert(counter_u32.unwrap_or(99) == 99);

            counter_u32 = 42;
            assert(counter_u32.unwrap() == 42);

            counter_u32 = none;
            assert(counter_u32 == none);
            assert(counter_u32.unwrap_or(7) == 7);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Primitive none behavior
    // ────────────────────────────────────────────────────────────────
    //
    async transition primitives_none_behavior() -> Future {
        return async {
            assert(flag == none);
            assert(flag.unwrap_or(true) == true);

            flag = false;
            assert(flag.unwrap() == false);

            flag = none;
            assert(flag == none);
            assert(flag.unwrap_or(false) == false);

            scalar_val = none;
            field_val = none;
            assert(scalar_val == none);
            assert(field_val == none);

            scalar_val = 1scalar;
            field_val = 5field;
            assert(scalar_val.unwrap() == 1scalar);
            assert(field_val.unwrap() == 5field);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Arrays with none elements (local only)
    // ────────────────────────────────────────────────────────────────
    //
    async transition arrays_none_behavior() -> Future {
        return async {
            let arr: [u32?; 3] = [none, 2, none];
            assert(arr[0].unwrap_or(9) == 9);
            assert(arr[1].unwrap_or(0) == 2);
            assert(arr[2].unwrap_or(4) == 4);

            let p0: Point? = none;
            let p1: Point? = Point { x: 1field, y: 2field };
            let locals: [Point?; 2] = [p0, p1];

            assert(locals[0] == none);
            assert(locals[1].unwrap().x == 1field);

            points = [Point { x: 0field, y: 0field }, Point { x: 9field, y: 9field }];
            assert(points.unwrap()[1].x == 9field);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Structs with local optionals
    // ────────────────────────────────────────────────────────────────
    //
    async transition structs_none_behavior() -> Future {
        return async {
            // Local optional struct
            let c: Container? = none;
            assert(c == none);
            assert(c.unwrap_or(Container { id: 0, point: Point { x: 0field, y: 0field } }).id == 0);

            let local_container = Container {
                id: 1,
                point: Point { x: 5field, y: 10field },
            };
            assert(local_container.point.x == 5field);

            let maybe_c: Container? = local_container;
            assert(maybe_c.unwrap().point.y == 10field);

            container = Container { id: 9, point: Point { x: 9field, y: 9field } };
            assert(container.unwrap().point.x == 9field);
        };
    }

    @noupgrade
    async constructor() {}
}
