// Compilable scaffold so the snippet from `documentation/language/programs_in_practice/limitations.md`
// builds. Anchors carve out the parts shown in the doc.
program limitations_demo.aleo {
    @noupgrade
    constructor() {}

    // ANCHOR: weird_sub
    fn weird_sub(a: u8, b: u8) -> u8 {
        if (a >= b) {
            return a.sub_wrapped(b);
        } else {
            return b.sub_wrapped(a);
        }
    }
    // ANCHOR_END: weird_sub
}
