// Test: a submodule interface inherits from a top-level library interface,
// and another submodule interface inherits across modules within the same library.

// --- library: shapes --- //

interface Shape {
    fn area() -> u32;
}

// --- Next Module: polygons.leo --- //

interface Polygon : shapes::Shape {
    fn perimeter() -> u32;
}

// --- Next Module: quadrilaterals.leo --- //

interface Quadrilateral : shapes::polygons::Polygon {
    fn side_count() -> u32;
}

// --- Next Program --- //

program main.aleo : shapes::quadrilaterals::Quadrilateral {
    fn area() -> u32 {
        return 100u32;
    }

    fn perimeter() -> u32 {
        return 40u32;
    }

    fn side_count() -> u32 {
        return 4u32;
    }

    @noupgrade
    constructor() {}
}
