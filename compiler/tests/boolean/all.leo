// !(true && (false || true))
function main() {
    const a = true;
    const b = false || a;
    const c = !(true && b);

    assert_eq!(c, false);
}