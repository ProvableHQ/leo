export struct OptionalLookalike {
    is_some: bool,
    val: u8,
}

program test.aleo {
    storage data: [u8];
    mapping result: u32 => u8;

    fn zero_fallback(index: u32) -> Final {
        return final {
            let value: u8 = data.get(index).unwrap_or(0u8);
            result.set(0u32, value);
        };
    }

    fn nonzero_fallback(index: u32) -> Final {
        return final {
            let value: u8 = data.get(index).unwrap_or(7u8);
            result.set(1u32, value);
        };
    }

    fn input_fallback(index: u32, fallback: u8) -> Final {
        return final {
            let value: u8 = data.get(index).unwrap_or(fallback);
            result.set(2u32, value);
        };
    }

    fn dynamic_output_key(index: u32, key: u32) -> Final {
        return final {
            let value: u8 = data.get(index).unwrap_or(0u8);
            result.set(key, value);
        };
    }

    fn structural_lookalike(condition: bool, value: u8) -> Final {
        return final {
            let selected: u8 = condition ? value : 0u8;
            let wrapper: OptionalLookalike = OptionalLookalike { is_some: condition, val: selected };
            let alias: OptionalLookalike = wrapper;
            let output: u8 = alias.is_some ? alias.val : 0u8;
            result.set(3u32, output);
        };
    }

    @noupgrade
    constructor() {}
}
