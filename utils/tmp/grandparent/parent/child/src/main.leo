// The 'a' program.
program child.aleo {
    record A {
        owner: address,
        val: u32,
    }
    transition mint(owner: address, val: u32) -> A {
        return A {owner: owner, val: val};
    }
}
