// Returns x * 4 by applying base_lib::double twice.
// This crosses a library boundary: calc_lib calls base_lib.
fn quadruple(x: u32) -> u32 {
    return base_lib::double(base_lib::double(x));
}
