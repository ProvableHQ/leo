program test.aleo {
    struct Point {
        x: u32,
        y: u32,
    }

    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    struct Entity {
        id: u32,
        health: u32,
        active: bool,
    }

    // Stresss test with a mx of structs, tuples, arrays, and scalars with 50+ captures
    async transition main() -> Future {
        // 10 Point structs
        let p0 = Point { x: 0u32, y: 10u32 };
        let p1 = Point { x: 1u32, y: 11u32 };
        let p2 = Point { x: 2u32, y: 12u32 };
        let p3 = Point { x: 3u32, y: 13u32 };
        let p4 = Point { x: 4u32, y: 14u32 };
        let p5 = Point { x: 5u32, y: 15u32 };
        let p6 = Point { x: 6u32, y: 16u32 };
        let p7 = Point { x: 7u32, y: 17u32 };
        let p8 = Point { x: 8u32, y: 18u32 };
        let p9 = Point { x: 9u32, y: 19u32 };

        // 5 Color structs
        let c0 = Color { r: 255u8, g: 0u8, b: 0u8 };
        let c1 = Color { r: 0u8, g: 255u8, b: 0u8 };
        let c2 = Color { r: 0u8, g: 0u8, b: 255u8 };
        let c3 = Color { r: 128u8, g: 128u8, b: 0u8 };
        let c4 = Color { r: 0u8, g: 128u8, b: 128u8 };

        // 5 Entity structs
        let e0 = Entity { id: 100u32, health: 100u32, active: true };
        let e1 = Entity { id: 101u32, health: 80u32, active: true };
        let e2 = Entity { id: 102u32, health: 60u32, active: false };
        let e3 = Entity { id: 103u32, health: 40u32, active: true };
        let e4 = Entity { id: 104u32, health: 20u32, active: false };

        // 5 tuples
        let t0 = (0u32, 1u32);
        let t1 = (2u32, 3u32);
        let t2 = (4u32, 5u32);
        let t3 = (6u32, 7u32);
        let t4 = (8u32, 9u32);

        // 3 arrays
        let arr0 = [10u32, 11u32, 12u32];
        let arr1 = [13u32, 14u32, 15u32];
        let arr2 = [16u32, 17u32, 18u32];

        // 20 scalar u32 variables
        let s0 = 0u32;
        let s1 = 1u32;
        let s2 = 2u32;
        let s3 = 3u32;
        let s4 = 4u32;
        let s5 = 5u32;
        let s6 = 6u32;
        let s7 = 7u32;
        let s8 = 8u32;
        let s9 = 9u32;
        let s10 = 10u32;
        let s11 = 11u32;
        let s12 = 12u32;
        let s13 = 13u32;
        let s14 = 14u32;
        let s15 = 15u32;
        let s16 = 16u32;
        let s17 = 17u32;
        let s18 = 18u32;
        let s19 = 19u32;

        // Total: 10 Points + 5 Colors + 5 Entities + 5 Tuples + 3 Arrays + 20 Scalars = 48 captures
        // This will require multiple bundle structs
        let f = async {
            // Use Point structs
            let sum_x = p0.x + p1.x + p2.x + p3.x + p4.x + p5.x + p6.x + p7.x + p8.x + p9.x;
            let sum_y = p0.y + p1.y + p2.y + p3.y + p4.y + p5.y + p6.y + p7.y + p8.y + p9.y;
            
            // Use Color structs
            let sum_r = (c0.r as u32) + (c1.r as u32) + (c2.r as u32) + (c3.r as u32) + (c4.r as u32);
            let sum_g = (c0.g as u32) + (c1.g as u32) + (c2.g as u32) + (c3.g as u32) + (c4.g as u32);
            
            // Use Entity structs
            let sum_health = e0.health + e1.health + e2.health + e3.health + e4.health;
            let active_count = (e0.active ? 1u32 : 0u32) + (e1.active ? 1u32 : 0u32) 
                             + (e2.active ? 1u32 : 0u32) + (e3.active ? 1u32 : 0u32) 
                             + (e4.active ? 1u32 : 0u32);
            
            // Use tuples
            let sum_tuples = t0.0 + t0.1 + t1.0 + t1.1 + t2.0 + t2.1 + t3.0 + t3.1 + t4.0 + t4.1;
            
            // Use arrays
            let sum_arrays = arr0[0u32] + arr0[1u32] + arr1[0u32] + arr1[1u32] + arr2[0u32] + arr2[1u32];
            
            // Use scalars
            let sum_scalars = s0 + s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9
                            + s10 + s11 + s12 + s13 + s14 + s15 + s16 + s17 + s18 + s19;
            
            // Verify all values
            assert_eq(sum_x, 45u32);
            assert_eq(sum_y, 145u32);
            assert_eq(sum_r, 511u32);
            assert_eq(sum_g, 511u32);
            assert_eq(sum_health, 300u32);
            assert_eq(active_count, 3u32);
            assert_eq(sum_tuples, 45u32);
            assert_eq(sum_arrays, 81u32);
            assert_eq(sum_scalars, 190u32);
        };
        return f;
    }
}

