// Leo standard library.
//
// The `std` library is implicitly available in every Leo program; there is no
// need to declare it in `program.json` or to import it explicitly. Functions
// are reached through their qualified path, e.g. `std::hash::bhp256::hash_u64_to_field(x)`
// or `std::ctx::caller()`.
//
// A program can opt out of the implicit `std` injection by setting
// `"no_std": true` in its `program.json`.
