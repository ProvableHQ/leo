import test-import.(
    Point,
    foo
);

function main() {
    const a = Point { x: 1u32, y: 0u32 };

    console.assert(a.x == 1u32);
}