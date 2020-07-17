// !(true && (false || true))
function main() -> bool {
    const a = true;
    const b = false || a;
    const c = !(true && b);

    return c
}