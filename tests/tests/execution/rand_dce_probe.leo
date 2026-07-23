/*
Test that unused random draws are still preserved.

[case]
program = "rand_dce_probe.aleo"
function = "with_unused"
input = []

[case]
program = "rand_dce_probe.aleo"
function = "without_unused"
input = []

[case]
program = "rand_dce_probe.aleo"
function = "with_used"
input = []
*/

program rand_dce_probe.aleo {
    mapping values: u8 => u8;

    fn with_unused() -> Final {
        return final { finalize_with_unused(); };
    }

    fn without_unused() -> Final {
        return final { finalize_without_unused(); };
    }

    fn with_used() -> Final {
        return final { finalize_with_used(); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_with_unused() {
    let unused: u8 = ChaCha::rand_u8();
    let kept: u8 = ChaCha::rand_u8();
    values.set(0u8, kept);
}

export final fn finalize_without_unused() {
    let kept: u8 = ChaCha::rand_u8();
    values.set(0u8, kept);
}

export final fn finalize_with_used() {
    let used: u8 = ChaCha::rand_u8();
    let kept: u8 = ChaCha::rand_u8();
    values.set(1u8, used);
    values.set(0u8, kept);
}
