import bar.Bar as Baz;

circuit Bar {
    r: bool,
}
function main() {
    let z = Baz { z: 0u32 };
    let r = Bar { r: true };
}