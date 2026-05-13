// ANCHOR: tests
program test_math_utils.aleo {
    @test
    fn test_abs() {
        assert_eq(math_utils::abs(0i32 - 5i32), 5i32);
    }

    @test
    fn test_geometry_area() {
        assert_eq(math_utils::geometry::area(3u32, 4u32), 12u32);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: tests
