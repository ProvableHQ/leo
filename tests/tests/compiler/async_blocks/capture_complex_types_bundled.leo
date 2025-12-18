program test.aleo {
    struct Position {
        x: u32,
        y: u32,
    }

    struct Entity {
        pos: Position,
        health: u32,
    }

    // Test capturing complex types (nested structs, arrays) with bundling
    async transition main() -> Future {
        let entity1 = Entity {
            pos: Position { x: 10u32, y: 20u32 },
            health: 100u32,
        };
        
        let entity2 = Entity {
            pos: Position { x: 30u32, y: 40u32 },
            health: 80u32,
        };
        
        let arr1 = [1u32, 2u32, 3u32];
        let arr2 = [4u32, 5u32, 6u32];
        let arr3 = [7u32, 8u32, 9u32];
        
        // Additional scalars to push over 16 captures
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

        // Total captures: 2 entities + 3 arrays + 18 scalars = 23 captures
        // Should create 1 bundle with 7 fields
        let f = async {
            // Use nested struct fields
            assert_eq(entity1.pos.x, 10u32);
            assert_eq(entity1.pos.y, 20u32);
            assert_eq(entity1.health, 100u32);
            assert_eq(entity2.pos.x, 30u32);
            assert_eq(entity2.health, 80u32);
            
            // Use arrays
            assert_eq(arr1[0u32], 1u32);
            assert_eq(arr2[1u32], 5u32);
            assert_eq(arr3[2u32], 9u32);
            
            // Use scalars
            let scalar_sum = s0 + s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 
                           + s10 + s11 + s12 + s13 + s14 + s15 + s16 + s17;
            assert_eq(scalar_sum, 153u32);
        };
        return f;
    }
}

