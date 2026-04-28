program my_app.aleo {
    fn normalize(x: u32) -> u32 {
        return math_utils::clamp::[100u32](x);
    }

    @noupgrade
    constructor() {}
}
