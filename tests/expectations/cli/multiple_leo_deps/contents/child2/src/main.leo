import grandchild.aleo;

program child2.aleo {
    record R {
        owner: address,
        f2: field
    }

    transition main(b: u32) -> u32 {
        return grandchild.aleo/main(b);
    }

    @noupgrade
    async constructor() {}
}
