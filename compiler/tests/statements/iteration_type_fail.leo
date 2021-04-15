// code sample is from fuzzing bug
// https://github.com/AleoHQ/leo/issues/758
function main (
    x: u8,
) {
    for y in 0u8..10u8 {
        let z = y > x;
    }
}
