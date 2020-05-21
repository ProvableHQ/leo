// !(true && (false || true))
function main() -> bool {
  let a = true;
  let b = false || a;
  let c = !(true && b);

  return c
}