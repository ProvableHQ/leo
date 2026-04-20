program my_app.aleo {
    // Struct from library submodule in parameter and return position.
    fn rect_area(r: shapes_lib::geometry::Rect) -> u32 {
        return shapes_lib::geometry::area(r);
    }

    // Build a Rect from individual coordinates; uses nested submodule struct.
    fn make_rect(x0: u32, y0: u32, x1: u32, y1: u32) -> shapes_lib::geometry::Rect {
        return shapes_lib::geometry::Rect {
            top_left: shapes_lib::geometry::Point { x: x0, y: y0 },
            bottom_right: shapes_lib::geometry::Point { x: x1, y: y1 },
        };
    }

    // Submodule constant in expression.
    fn unit_side() -> u32 {
        return shapes_lib::geometry::UNIT;
    }

    // Scale area by a factor using the top-level library function.
    fn scaled_area(r: shapes_lib::geometry::Rect, factor: u32) -> u32 {
        return shapes_lib::scale(shapes_lib::geometry::area(r), factor);
    }

    @noupgrade
    constructor() {}
}
