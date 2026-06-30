// Submodule: a small helper kept in its own file so the library's `modules` map is
// non-empty (its keys are module paths, which the snapshot serializer must handle).

export const UNIT: u32 = 1u32;

export fn increment(x: u32) -> u32 {
    return x + UNIT;
}
