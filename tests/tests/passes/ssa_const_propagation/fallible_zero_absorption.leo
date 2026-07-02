program test.aleo {
    @noupgrade
    constructor() {}

    fn preserve_checked_sub_right(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        return diff * 0u8;
    }

    fn preserve_checked_sub_left(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        return 0u8 * diff;
    }

    fn preserve_checked_sub_alias(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        let alias: u8 = diff;
        return alias * 0u8;
    }

    fn fold_plain_parameter_zero(x: u8) -> u8 {
        return x * 0u8;
    }

    fn keep_checked_sub_ternary(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        return true ? 0u8 : diff;
    }
}
