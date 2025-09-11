/*
seed = 123456789

[case]
program = "test_assignments.aleo"
function = "run_transmogrify"
input = ["1234field", "12u8", "true"]
[case]
program = "test_assignments.aleo"
function = "run_transmogrify"
input = ["12345678field", "100u8", "false"]
*/


program test_assignments.aleo {
    struct T {
        x: field,
        y: u8,
        z: bool,
    }

    struct S {
        x: [T; 2],
        y: field,
    }

    function transmogrify(s: S, a: field, b: field) -> S {
        for i in 0u8..7u8 {
            s.x[0u32].x += a;
            s.x[1u32].x += b;
        }

        s.y += b;
        s.x[0u32].z = !s.x[0u32].z;

        return s;
    }

    transition run_transmogrify(x: field, y: u8, z: bool) -> S {
        let t1 = T {
            x,
            y,
            z,
        };

        let t2 = T {
            x: x + y as field,
            y: y + 1u8,
            z: !z,
        };

        let s = S {
            x: [t1, t2],
            y: x,
        };

        return transmogrify(s, x + 1field, x + 12field);
    }

    @noupgrade
    async constructor() {}
}
