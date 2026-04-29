// ANCHOR: file
interface Base {
    fn get_value() -> u64;
}

interface Extended : Base {
    fn set_value(v: u64) -> u64;
}
// ANCHOR_END: file

program inheritance_demo.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}
