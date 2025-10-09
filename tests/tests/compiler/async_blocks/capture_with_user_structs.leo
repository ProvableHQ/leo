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
        active: bool,
    }

    // Test capturing user-defined structs along with many scalar variables
    // This ensures synthetic bundle structs don't conflict with user structs
    async transition main() -> Future {
        let p1 = Point { x: 10u32, y: 20u32 };
        let p2 = Point { x: 30u32, y: 40u32 };
        let c1 = Color { r: 255u8, g: 0u8, b: 0u8 };
        let e1 = Entity { id: 1u32, active: true };
        
        // Add scalar variables to exceed 16 captures
        let a0 = 0u32;
        let a1 = 1u32;
        let a2 = 2u32;
        let a3 = 3u32;
        let a4 = 4u32;
        let a5 = 5u32;
        let a6 = 6u32;
        let a7 = 7u32;
        let a8 = 8u32;
        let a9 = 9u32;
        let b0 = 10u32;
        let b1 = 11u32;
        let b2 = 12u32;
        let b3 = 13u32;
        let b4 = 14u32;
        let b5 = 15u32;
        let b6 = 16u32;
        let b7 = 17u32;

        // Total: 4 structs + 18 scalars = 22 captures
        // Should create 1 bundle struct
        let f = async {
            assert_eq(p1.x, 10u32);
            assert_eq(p1.y, 20u32);
            assert_eq(p2.x, 30u32);
            assert_eq(p2.y, 40u32);
            assert_eq(c1.r, 255u8);
            assert(e1.active);
            
            let sum = a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9
                    + b0 + b1 + b2 + b3 + b4 + b5 + b6 + b7;
            assert_eq(sum, 153u32);
        };
        return f;
    }
}

