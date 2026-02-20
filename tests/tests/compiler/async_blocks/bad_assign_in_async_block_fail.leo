struct Foo { x: u32, y: u32 }

program test.aleo {
    fn foo(p: u32, q: u8) -> Final {
        let y = (1u32, 2u8, 3u16);
        let a = [1u32, 2u32];
        let x = 0u32;
        let s = Foo { x: 9, y: 4 };
        let f = final {
            // All of these are bad
            y = (1u32, 2u8, 3u16);
            y.0 = 1u32;
            a = [1u32, 2u32];
            a[0] = 3;
            x = 2;
            s.x = 99;

            {
                {
                    y = (1u32, 2u8, 3u16);
                    y.0 = 1u32;
                    a = [1u32, 2u32];
                    a[0] = 3;
                    x = 2;
                    s.x = 99;
                }
            }
        };
        return f;
    }
}

