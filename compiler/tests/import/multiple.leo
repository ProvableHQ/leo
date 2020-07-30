import test_import.(
    Point,
    foo
);

function main() {
    let a = Point { x: 1u32, y: 0u32 };

    assert_eq!(a.x, 1u32);
}