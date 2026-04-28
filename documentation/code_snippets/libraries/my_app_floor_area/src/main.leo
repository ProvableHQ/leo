program my_app.aleo {
    fn floor_area(w: u32, h: u32) -> u32 {
        return math_utils::geometry::area(w, h);
    }

    @noupgrade
    constructor() {}
}
