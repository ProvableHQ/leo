program test.aleo {
    @noupgrade
    constructor() {}

    storage nums: [u32];

    fn unused_get(x: u32) -> Final {
        return final { nums.get(1u32 / x); };
    }
}
