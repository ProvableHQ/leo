*/

program test.aleo {
    mapping map: u32 => u32;

    async transition main() -> Future {
        return finalize_main(1u32);
    }

    async function finalize_main(a:u32) {
        if a == 1u32 {
            let y: u32 = 1u32;
            {
                // This is OK, but previously yielded an error.
                y = 2u32;
            }
            map.set(y, y);
        }
    }
}
