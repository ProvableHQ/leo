import test_import.(
  Point,
  foo
);

function main() -> u32 {
  let p = Point { x: 1u32, y: 0u32 };
  return p.x
}