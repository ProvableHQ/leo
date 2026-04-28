const CEILING: u32 = math_utils::MAX_U32;

// ANCHOR: program
program my_app.aleo {
    fn closest(
        origin: math_utils::Point,
        a: math_utils::Point,
        b: math_utils::Point,
    ) -> math_utils::Point {
        let da: u32 = math_utils::manhattan(origin, a);
        let db: u32 = math_utils::manhattan(origin, b);
        return da <= db ? a : b;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: program
