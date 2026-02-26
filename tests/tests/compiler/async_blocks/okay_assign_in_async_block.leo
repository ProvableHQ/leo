struct Foo { x: u32, y: u32 }

program test.aleo {
    fn foo(p: u32, q: u8) -> Final {
        let f = final {
            let y = (1u32, 2u8, 3u16);
            let a = [1u32, 2u32];
            let x = 0u32;
            let s = Foo { x: 9, y: 4 };

            // All of these are okay since the variables are declared inside the block
            y = (1u32, 2u8, 3u16);
            y.0 = 1u32;
            a = [1u32, 2u32];
            a[0] = 3;
            x = 2;
            s.x = 99;

            let w = 0u32;
            {
                w = 1;
            }

            {
                let b = 0u32;
                {
                    b = 1;
                }
            }

            {
                {
                    let c = 0u32;
                    c = 1;
                }
            }

            for i in 0u32..2u32 {
                let d = 0u32;
                d = 1;
            }
        };
        return f;
    }
}

