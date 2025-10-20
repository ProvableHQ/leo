/*
seed = 987654321
min_height = 16

[case]
program = "mixed_types.aleo"
function = "unsigned_integers"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "mixed_types.aleo"
function = "signed_integers"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "mixed_types.aleo"
function = "primitive_types"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "mixed_types.aleo"
function = "structs_and_arrays"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program mixed_types.aleo {
    //
    // ────────────────────────────────────────────────────────────────
    // Storage declarations
    // ────────────────────────────────────────────────────────────────
    //
    storage counter_u32: u32;
    storage counter_i32: i32;
    storage flag: bool;
    storage scalar_val: scalar;
    storage field_val: field;
    storage group_val: group;
    storage containers: [Container; 2];
    storage points_array: [Point; 3];

    struct Point {
        x: field,
        y: field,
    }

    struct Container {
        points: [Point; 2],
        label: u8,
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Unsigned integers test
    // ────────────────────────────────────────────────────────────────
    //
    async transition unsigned_integers() -> Future {
        return async {
            counter_u32 = 10;
            assert(counter_u32.unwrap() == 10);

            let new_val = counter_u32.unwrap_or(0) + 5;
            counter_u32 = new_val;
            assert(counter_u32.unwrap() == 15);

            // Confirm unwrap_or fallback
            let temp = counter_u32.unwrap_or(99);
            assert(temp == 15);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Signed integers test
    // ────────────────────────────────────────────────────────────────
    //
    async transition signed_integers() -> Future {
        return async {
            counter_i32 = -5i32;
            assert(counter_i32.unwrap() == -5i32);

            let new_val = counter_i32.unwrap_or(0) - 3;
            counter_i32 = new_val;
            assert(counter_i32.unwrap() == -8i32);

            // Check fallback
            let fallback = counter_i32.unwrap_or(1);
            assert(fallback == -8i32);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Primitive types test
    // ────────────────────────────────────────────────────────────────
    //
    async transition primitive_types() -> Future {
        return async {
            flag = true;
            scalar_val = 2scalar;
            field_val = 21field;
            group_val = 0group;

            assert(flag.unwrap() == true);
            assert(scalar_val.unwrap() == 2scalar);
            assert(field_val.unwrap() == 21field);
            assert(group_val.unwrap() == 0group);

            // Flip and recheck
            flag = !flag.unwrap();
            assert(flag.unwrap() == false);

            // Ensure unwrap_or fallback doesn't modify behavior
            assert(flag.unwrap_or(true) == false);
            assert(field_val.unwrap_or(0field) == 21field);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Structs and arrays test (nested)
    // ────────────────────────────────────────────────────────────────
    //
    async transition structs_and_arrays() -> Future {
        return async {
            // Build nested data
            let p0 = Point { x: 1field, y: 2field };
            let p1 = Point { x: 3field, y: 4field };
            let p2 = Point { x: 5field, y: 6field };

            let c0 = Container { points: [p0, p1], label: 10 };
            let c1 = Container { points: [p2, p0], label: 20 };

            containers = [c0, c1];
            points_array = [p0, p1, p2];

            // Validate values immediately
            let c = containers.unwrap();
            assert(c[0].points[0].x == 1field);
            assert(c[1].points[0].y == 6field);
            assert(c[0].label == 10);
            assert(c[1].label == 20);

            // Mutate and check again
            let c_mut = c;
            c_mut[0].points[0].x = 9field;
            containers = c_mut;

            let c2 = containers.unwrap();
            assert(c2[0].points[0].x == 9field);
            assert(c2[1].points[0].x == 5field);

            // Check nested array of structs
            let arr = points_array.unwrap();
            assert(arr[0].x == 1field);
            assert(arr[2].y == 6field);

            let arr2 = arr;
            arr2[1].x = arr2[1].x + 10field;
            points_array = arr2;

            let arr3 = points_array.unwrap();
            assert(arr3[1].x == 13field);
        };
    }

    @noupgrade
    async constructor() {}
}
