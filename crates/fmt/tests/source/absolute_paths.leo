fn helper(s: ::c::Shared) -> u32 {
    let v = ::c::shared( 1u32 ).val;
    let s2 = ::c::Shared { val:  ::c::X };
    let g = ::c::Gen::[2] { vals: [1u32,  2u32] };
    return v + s2.val + g.vals[0u32];
}
