program test.aleo {
    async function bar() {
        for i in 1u8..2u16 {
            for j: u8 in 0u32..1u32 {

            }
        }

        for i in 1field..2scalar {

        }
    }

    async transition main() -> Future {
        return bar();
    }
}
