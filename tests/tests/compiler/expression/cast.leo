program test.aleo {
    // The primitive types are: `address`, `bool`, `field`, `group`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `scalar`.
    // We can cast between all of these types.
    fn main(
        a: address,
        b: bool,
        c: field,
        d: i8,
        e: i16,
        f: i64,
        g: i128,
        h: u8,
        i: u16,
        j: u32,
        k: u64,
        l: u128,
        m: scalar,
    ) -> bool {
        let _a_field: field = a as field;
        let _a_group: group = a as group;
        let _a_bool: bool = a as bool;
        let _a_i8: i8 = a as i8;
        let _a_i16: i16 = a as i16;
        let _a_i32: i32 = a as i32;
        let _a_i64: i64 = a as i64;
        let _a_i128: i128 = a as i128;
        let _a_u8: u8 = a as u8;
        let _a_u16: u16 = a as u16;
        let _a_u32: u32 = a as u32;
        let _a_u64: u64 = a as u64;
        let _a_u128: u128 = a as u128;
        let _a_scalar: scalar = a as scalar;

        let _b_field: field = b as field;
        let _b_group: group = b as group;
        let _b_bool: bool = b as bool;
        let _b_i8: i8 = b as i8;
        let _b_i16: i16 = b as i16;
        let _b_i32: i32 = b as i32;
        let _b_i64: i64 = b as i64;
        let _b_i128: i128 = b as i128;
        let _b_u8: u8 = b as u8;
        let _b_u16: u16 = b as u16;
        let _b_u32: u32 = b as u32;
        let _b_u64: u64 = b as u64;
        let _b_u128: u128 = b as u128;
        let _b_scalar: scalar = b as scalar;

        let _c_field: field = c as field;
        let _c_group: group = c as group;
        let _c_bool: bool = c as bool;
        let _c_i8: i8 = c as i8;
        let _c_i16: i16 = c as i16;
        let _c_i32: i32 = c as i32;
        let _c_i64: i64 = c as i64;
        let _c_i128: i128 = c as i128;
        let _c_u8: u8 = c as u8;
        let _c_u16: u16 = c as u16;
        let _c_u32: u32 = c as u32;
        let _c_u64: u64 = c as u64;
        let _c_u128: u128 = c as u128;
        let _c_scalar: scalar = c as scalar;

        let _d_field: field = d as field;
        let _d_group: group = d as group;
        let _d_bool: bool = d as bool;
        let _d_i8: i8 = d as i8;
        let _d_i16: i16 = d as i16;
        let _d_i32: i32 = d as i32;
        let _d_i64: i64 = d as i64;
        let _d_i128: i128 = d as i128;
        let _d_u8: u8 = d as u8;
        let _d_u16: u16 = d as u16;
        let _d_u32: u32 = d as u32;
        let _d_u64: u64 = d as u64;
        let _d_u128: u128 = d as u128;
        let _d_scalar: scalar = d as scalar;

        let _e_field: field = e as field;
        let _e_group: group = e as group;
        let _e_bool: bool = e as bool;
        let _e_i8: i8 = e as i8;
        let _e_i16: i16 = e as i16;
        let _e_i32: i32 = e as i32;
        let _e_i64: i64 = e as i64;
        let _e_i128: i128 = e as i128;
        let _e_u8: u8 = e as u8;
        let _e_u16: u16 = e as u16;
        let _e_u32: u32 = e as u32;
        let _e_u64: u64 = e as u64;
        let _e_u128: u128 = e as u128;
        let _e_scalar: scalar = e as scalar;

        let _f_field: field = f as field;
        let _f_group: group = f as group;
        let _f_bool: bool = f as bool;
        let _f_i8: i8 = f as i8;
        let _f_i16: i16 = f as i16;
        let _f_i32: i32 = f as i32;
        let _f_i64: i64 = f as i64;
        let _f_i128: i128 = f as i128;
        let _f_u8: u8 = f as u8;
        let _f_u16: u16 = f as u16;
        let _f_u32: u32 = f as u32;
        let _f_u64: u64 = f as u64;
        let _f_u128: u128 = f as u128;
        let _f_scalar: scalar = f as scalar;

        let _g_field: field = g as field;
        let _g_group: group = g as group;
        let _g_bool: bool = g as bool;
        let _g_i8: i8 = g as i8;
        let _g_i16: i16 = g as i16;
        let _g_i32: i32 = g as i32;
        let _g_i64: i64 = g as i64;
        let _g_i128: i128 = g as i128;
        let _g_u8: u8 = g as u8;
        let _g_u16: u16 = g as u16;
        let _g_u32: u32 = g as u32;
        let _g_u64: u64 = g as u64;
        let _g_u128: u128 = g as u128;
        let _g_scalar: scalar = g as scalar;

        let _h_field: field = h as field;
        let _h_group: group = h as group;
        let _h_bool: bool = h as bool;
        let _h_i8: i8 = h as i8;
        let _h_i16: i16 = h as i16;
        let _h_i32: i32 = h as i32;
        let _h_i64: i64 = h as i64;
        let _h_i128: i128 = h as i128;
        let _h_u8: u8 = h as u8;
        let _h_u16: u16 = h as u16;
        let _h_u32: u32 = h as u32;
        let _h_u64: u64 = h as u64;
        let _h_u128: u128 = h as u128;
        let _h_scalar: scalar = h as scalar;

        let _i_field: field = i as field;
        let _i_group: group = i as group;
        let _i_bool: bool = i as bool;
        let _i_i8: i8 = i as i8;
        let _i_i16: i16 = i as i16;
        let _i_i32: i32 = i as i32;
        let _i_i64: i64 = i as i64;
        let _i_i128: i128 = i as i128;
        let _i_u8: u8 = i as u8;
        let _i_u16: u16 = i as u16;
        let _i_u32: u32 = i as u32;
        let _i_u64: u64 = i as u64;
        let _i_u128: u128 = i as u128;
        let _i_scalar: scalar = i as scalar;

        let _j_field: field = j as field;
        let _j_group: group = j as group;
        let _j_bool: bool = j as bool;
        let _j_i8: i8 = j as i8;
        let _j_i16: i16 = j as i16;
        let _j_i32: i32 = j as i32;
        let _j_i64: i64 = j as i64;
        let _j_i128: i128 = j as i128;
        let _j_u8: u8 = j as u8;
        let _j_u16: u16 = j as u16;
        let _j_u32: u32 = j as u32;
        let _j_u64: u64 = j as u64;
        let _j_u128: u128 = j as u128;
        let _j_scalar: scalar = j as scalar;

        let _k_field: field = k as field;
        let _k_group: group = k as group;
        let _k_bool: bool = k as bool;
        let _k_i8: i8 = k as i8;
        let _k_i16: i16 = k as i16;
        let _k_i32: i32 = k as i32;
        let _k_i64: i64 = k as i64;
        let _k_i128: i128 = k as i128;
        let _k_u8: u8 = k as u8;
        let _k_u16: u16 = k as u16;
        let _k_u32: u32 = k as u32;
        let _k_u64: u64 = k as u64;
        let _k_u128: u128 = k as u128;

        let l_field: field = l as field;
        let _l_group: group = l as group;
        let _l_bool: bool = l as bool;
        let _l_i8: i8 = l as i8;
        let _l_i16: i16 = l as i16;
        let _l_i32: i32 = l as i32;
        let _l_i64: i64 = l as i64;
        let _l_i128: i128 = l as i128;
        let _l_u8: u8 = l as u8;
        let _l_u16: u16 = l as u16;
        let _l_u32: u32 = l as u32;
        let _l_u64: u64 = l as u64;
        let _l_u128: u128 = l as u128;
        let _l_scalar: scalar = l as scalar;

        let m_field: field = m as field;
        let _m_group: group = m as group;
        let _m_bool: bool = m as bool;
        let _m_i8: i8 = m as i8;
        let _m_i16: i16 = m as i16;
        let _m_i32: i32 = m as i32;
        let _m_i64: i64 = m as i64;
        let _m_i128: i128 = m as i128;
        let _m_u8: u8 = m as u8;
        let _m_u16: u16 = m as u16;
        let _m_u32: u32 = m as u32;
        let _m_u64: u64 = m as u64;
        let _m_u128: u128 = m as u128;

        return l_field == m_field;
    }

    @noupgrade
    constructor() {}
}
