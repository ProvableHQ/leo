program my_app.aleo {
    @noupgrade
    constructor() {}

    // `src` genuinely depends on `mathlib` (its type and function are used here).
    fn combine(public a: u32, public b: u32) -> u32 {
        let p: mathlib::Pair = mathlib::Pair { a: a, b: b };
        return mathlib::sum(p);
    }
}
