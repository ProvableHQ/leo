import my_app.aleo;

program test_my_app.aleo {
    // rect_area: 3 × 4 = 12.
    @test
    fn test_rect_area() {
        let r = my_app.aleo::make_rect(0u32, 0u32, 3u32, 4u32);
        assert_eq(my_app.aleo::rect_area(r), 12u32);
    }

    // unit_side returns the submodule constant UNIT = 1.
    @test
    fn test_unit_side() {
        assert_eq(my_app.aleo::unit_side(), 1u32);
    }

    // scaled_area: 2×3 rect, factor 5 → area 6 × 5 = 30.
    @test
    fn test_scaled_area() {
        let r = my_app.aleo::make_rect(0u32, 0u32, 2u32, 3u32);
        assert_eq(my_app.aleo::scaled_area(r, 5u32), 30u32);
    }

    @noupgrade
    constructor() {}
}
