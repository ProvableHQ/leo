program test.aleo {
    @noupgrade
    constructor() {}

    fn preserve_false_arm(x: u8) -> u8 {
        let y: u8 = true ? x : x.div_wrapped(0u8);
        return y;
    }

    fn preserve_true_arm(x: u8) -> u8 {
        let y: u8 = false ? x.div_wrapped(0u8) : x;
        return y;
    }

    fn preserve_unwrap_arm(x: u8) -> u8 {
        let maybe: u8? = none;
        let y: u8 = true ? x : maybe.unwrap();
        return y;
    }

    fn fold_pure_arm(x: u8) -> u8 {
        let y: u8 = true ? x : x.add_wrapped(1u8);
        return y;
    }

    fn fold_const_initializer(x: u8) -> u8 {
        const N: u8 = true ? 5u8 : x.div_wrapped(1u8);
        return N;
    }
}
