// Regression: a different specialization is rejected in function, record, mapping,
// and storage interface types.

// --- library: generic_shapes --- //

export struct Slot::[N: u32] {
    amount: u32,
}

export interface Storer {
    record Token {
        owner: address,
        slot: Slot::[8u32],
        ..
    }

    mapping slots: address => Slot::[8u32];
    storage current: Slot::[8u32];

    fn store(input: Slot::[8u32]) -> Slot::[8u32];
}

// --- Next Program --- //

program consumer.aleo: generic_shapes::Storer {
    record Token {
        owner: address,
        slot: generic_shapes::Slot::[16u32],
    }

    mapping slots: address => generic_shapes::Slot::[16u32];
    storage current: generic_shapes::Slot::[16u32];

    fn store(input: generic_shapes::Slot::[16u32]) -> generic_shapes::Slot::[16u32] {
        return input;
    }

    @noupgrade
    constructor() {}
}
