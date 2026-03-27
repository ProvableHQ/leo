/*
seed = 123456789
min_height = 16

[case]
program = "setter_caller.aleo"
function = "main"
input = ["131094911852478662314321267field", "1u32", "100u32", "5u32"]
*/

interface Setter {
    fn set_val(key: u32, value: u32) -> (bool, Final);
}

program setter_impl.aleo: Setter {
    storage data: u32;

    fn set_val(key: u32, value: u32) -> (bool, Final) {
        return (
            true,
            final {
                data = value;
            },
        );
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import setter_impl.aleo;

program setter_caller.aleo {
    storage local_count: u32;

    fn main(target: field, key: u32, value: u32, extra: u32) -> (bool, Final) {
        let (ok, f): (bool, Final) = setter_impl.aleo::Setter@(target)/set_val(key, value);
        return (
            ok,
            final {
                f.run();
                local_count = local_count.unwrap_or(0u32) + extra;
            },
        );
    }

    @noupgrade
    constructor() {}
}
