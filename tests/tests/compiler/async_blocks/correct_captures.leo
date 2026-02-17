const X: u32 = 5;

program test.aleo {
    mapping map: u32 => u32;

    fn foo(p: u32, q: u8) -> Final {
        let x = 5u32;
        let y = (1u32, 2u8, 3u16);
        let w = (1u32, true);
        let z = [1u8; 3];
        let (t1, t2) = (6u32, 8u8);
        let f = final {
            let w2 = w;
            let sum1 = x + y.0 + (y.1 as u32) + (z[0] as u32) + (z[2] as u32) + w.0 + w2.0;
            let y2 = y;
            let sum1_again = x + y2.0 + (y2.1 as u32) + (z[0] as u32) + (z[2] as u32);
            let sum2 = p + (q as u32) + t1 + t2 as u32 + X;
            let total = sum1 + sum1_again + sum2;
            map.set(0u32, total); // `total` is local so it shouldn't be in the param list of the produced async function 

            {
                let total2 = sum1 + sum1_again + sum2;
                map.set(0u32, total2); // `total2` is local so it shouldn't be in the param list of the produced async function 
            }
        };

        // No shadowing
        let total = x;
        assert(total >= 0u32);
        let total2 = x;
        assert(total2 >= 0u32);


        return f;
    }
}
