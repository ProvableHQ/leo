program test.aleo {
    async transition main(x: u32) -> Future {
        let a0 = 0u32;
        let a1 = 0u32;
        let a2 = 0u32;
        let a3 = 0u32;
        let a4 = 0u32;
        let a5 = 0u32;
        let a6 = 0u32;
        let a7 = 0u32;
        let a8 = 0u32;
        let a9 = 0u32;
        let b0 = 0u32;
        let b1 = 0u32;
        let b2 = 0u32;
        let b3 = 0u32;
        let b4 = 0u32;
        let b5 = 0u32;
        let b6 = 0u32;
        let b7 = 0u32;
        let b8 = 0u32;
        let b9 = 0u32;

        // More captures than allowed in the VM
        let f = async {
            assert(a0 == 0);
            assert(a1 == 0);
            assert(a2 == 0);
            assert(a3 == 0);
            assert(a4 == 0);
            assert(a5 == 0);
            assert(a6 == 0);
            assert(a7 == 0);
            assert(a8 == 0);
            assert(a9 == 0);
            assert(b0 == 0);
            assert(b1 == 0);
            assert(b2 == 0);
            assert(b3 == 0);
            assert(b4 == 0);
            assert(b5 == 0);
            assert(b6 == 0);
            assert(b7 == 0);
            assert(b8 == 0);
            assert(b9 == 0);
        };
        return f;
    }
}
