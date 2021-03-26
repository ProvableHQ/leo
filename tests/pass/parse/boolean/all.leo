// !(true && (false || true))
function main() {
    const a = true;
    const b = false || a;
    const c = !(true && b);

    console.assert(c == false);
}