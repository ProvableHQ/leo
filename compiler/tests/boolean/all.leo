// !(true && (false || true))
function main() {
    let a = true;
    let b = false || a;
    let c = !(true && b);

    console.assert(c == false);
}