program test.aleo {
    mapping arr: u32 => [u32; 4];

    fn update(s: bool, a: [u32; 4], b: [u32; 4]) -> Final {
        return final {
            let chosen: [u32; 4] = s ? a : b;
            arr.set(0u32, chosen);
        };
    }
}
