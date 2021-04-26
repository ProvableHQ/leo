circuit Foo {}

function two() -> u8 {
  return 2u8;
}

function main() {
  const a = 1u8;
  const b = 1field;
  const c = 1group;
  const d = (0, 1)group;
  const e = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
  const f = two();
  const g = [0u8; (3, 2)];
  const h = [[0u8; 3]; 2];
  const i = [1u8, 1u8, 1u8];
  const j = true;
  const k = (1u8, 1u8);
  const l = (1u8, 1u8, true);
  const m = Foo {};
}