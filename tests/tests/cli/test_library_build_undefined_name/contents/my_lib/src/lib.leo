// `missing_fn` is not declared anywhere, so path resolution / name lookup
// must fail.
export fn uses_undefined() -> u32 {
    return missing_fn();
}
