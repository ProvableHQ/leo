/*
seed = 246801357
min_height = 16

[case]
program = "vector_structs.aleo"
function = "test_vector_primitives_behavior"
input = ["1u32", "25u32"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "vector_structs.aleo"
function = "test_vector_structs_behavior"
input = ["0u32"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "vector_structs.aleo"
function = "test_vector_containers_behavior"
input = ["1u32"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program vector_structs.aleo {
    //
    // ────────────────────────────────────────────────────────────────
    // Struct definitions
    // ────────────────────────────────────────────────────────────────
    //
    struct Point {
        x: field,
        y: field,
    }

    struct Container {
        id: u32,
        points: [Point; 2],
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Storage declarations (vectors only allowed here)
    // ────────────────────────────────────────────────────────────────
    //
    storage ids: [u32];
    storage points: [Point];
    storage containers: [Container];

    //
    // ────────────────────────────────────────────────────────────────
    // Test 1 — Vector of primitives
    // ────────────────────────────────────────────────────────────────
    //
    async transition test_vector_primitives_behavior(index: u32, new_value: u32) -> Future {
        return async {
            // Ensure empty at start
            ids.clear();
            assert(ids.len() == 0u32);

            // Push initial elements
            ids.push(10u32);
            ids.push(20u32);
            ids.push(30u32);
            assert(ids.len() == 3u32);

            // Dynamic index from input 
            let val_opt = ids.get(index);
            assert(val_opt.unwrap() == 20u32);

            let val = val_opt.unwrap();
            assert(val == 20u32);

            // Use input to set a new value dynamically
            ids.set(index, new_value);
            assert(ids.get(index).unwrap() == new_value);

            // Swap remove last element
            let removed = ids.swap_remove(2u32);
            assert(removed == 30u32);
            assert(ids.len() == 2u32);

            // Pop last element
            let popped = ids.pop().unwrap();
            assert(popped == new_value);
            assert(ids.len() == 1u32);

            // Clear vector
            ids.clear();
            assert(ids.len() == 0u32);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Test 2 — Vector of structs
    // ────────────────────────────────────────────────────────────────
    //
    async transition test_vector_structs_behavior(idx: u32) -> Future {
        return async {
            assert(points.len() == 0u32);

            points.push(Point { x: 1field, y: 2field });
            points.push(Point { x: 3field, y: 4field });
            assert(points.len() == 2u32);

            // Use index from input
            let p = points.get(idx).unwrap();
            assert(p.x + p.y == (1field + 2field) || p.x + p.y == (3field + 4field));

            // Mutate element using index input
            points.set(idx, Point { x: 9field, y: 9field });
            let updated = points.get(idx).unwrap();
            assert(updated.x == 9field);
            assert(updated.y == 9field);

            points.clear();
            assert(points.len() == 0u32);
        };
    }

    //
    // ────────────────────────────────────────────────────────────────
    // Test 3 — Vector of containers (structs with arrays)
    // ────────────────────────────────────────────────────────────────
    //
    async transition test_vector_containers_behavior(idx: u32) -> Future {
        return async {
            assert(containers.len() == 0u32);

            let c1 = Container {
                id: 1u32,
                points: [Point { x: 11field, y: 22field }, Point { x: 33field, y: 44field }],
            };

            let c2 = Container {
                id: 2u32,
                points: [Point { x: 55field, y: 66field }, Point { x: 77field, y: 88field }],
            };

            containers.push(c1);
            containers.push(c2);
            assert(containers.len() == 2u32);

            // Use index from input
            let target = containers.get(idx).unwrap();
            assert(target.id == 1u32 || target.id == 2u32);

            // Dynamically mutate element at input index
            containers.set(
                idx,
                Container {
                    id: 99u32,
                    points: [Point { x: 999field, y: 999field }, Point { x: 33field, y: 33field }],
                },
            );

            let updated = containers.get(idx).unwrap();
            assert(updated.id == 99u32);
            assert(updated.points[0].x == 999field);

            containers.clear();
            assert(containers.len() == 0u32);
        };
    }

    @noupgrade
    async constructor() {}
}
