program test.aleo {
    @noupgrade
    constructor() {}

    fn preserve_false_arm(x: u8) -> u8 {
        return true ? x : x.div_wrapped(0u8);
    }

    fn preserve_true_arm(x: u8) -> u8 {
        return false ? x.div_wrapped(0u8) : x;
    }
}
