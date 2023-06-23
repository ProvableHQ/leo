/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // The primitive types are: `address`, `bool`, `field`, `group`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `scalar`.
    // We can cast between all of these types.
    transition main(a: address, b: bool, c: field, d: i8, e: i16, f: i64, g: i128, h: u8, i: u16, j: u32, k: u64, l: u128, m: scalar) -> bool {
        let a_field: field = a as field;
        let a_group: group = a as group;
        let a_bool: bool = a as bool;
        let a_i8: i8 = a as i8;
        let a_i16: i16 = a as i16;
        let a_i32: i32 = a as i32;
        let a_i64: i64 = a as i64;
        let a_i128: i128 = a as i128;
        let a_u8: u8 = a as u8;
        let a_u16: u16 = a as u16;
        let a_u32: u32 = a as u32;
        let a_u64: u64 = a as u64;
        let a_u128: u128 = a as u128;
        let a_scalar: scalar = a as scalar;

        let b_field: field = b as field;
        let b_group: group = b as group;
        let b_bool: bool = b as bool;
        let b_i8: i8 = b as i8;
        let b_i16: i16 = b as i16;
        let b_i32: i32 = b as i32;
        let b_i64: i64 = b as i64;
        let b_i128: i128 = b as i128;
        let b_u8: u8 = b as u8;
        let b_u16: u16 = b as u16;
        let b_u32: u32 = b as u32;
        let b_u64: u64 = b as u64;
        let b_u128: u128 = b as u128;
        let b_scalar: scalar = b as scalar;

        let c_field: field = c as field;
        let c_group: group = c as group;
        let c_bool: bool = c as bool;
        let c_i8: i8 = c as i8;
        let c_i16: i16 = c as i16;
        let c_i32: i32 = c as i32;
        let c_i64: i64 = c as i64;
        let c_i128: i128 = c as i128;
        let c_u8: u8 = c as u8;
        let c_u16: u16 = c as u16;
        let c_u32: u32 = c as u32;
        let c_u64: u64 = c as u64;
        let c_u128: u128 = c as u128;
        let c_scalar: scalar = c as scalar;

        let d_field: field = d as field;
        let d_group: group = d as group;
        let d_bool: bool = d as bool;
        let d_i8: i8 = d as i8;
        let d_i16: i16 = d as i16;
        let d_i32: i32 = d as i32;
        let d_i64: i64 = d as i64;
        let d_i128: i128 = d as i128;
        let d_u8: u8 = d as u8;
        let d_u16: u16 = d as u16;
        let d_u32: u32 = d as u32;
        let d_u64: u64 = d as u64;
        let d_u128: u128 = d as u128;
        let d_scalar: scalar = d as scalar;

        let e_field: field = e as field;
        let e_group: group = e as group;
        let e_bool: bool = e as bool;
        let e_i8: i8 = e as i8;
        let e_i16: i16 = e as i16;
        let e_i32: i32 = e as i32;
        let e_i64: i64 = e as i64;
        let e_i128: i128 = e as i128;
        let e_u8: u8 = e as u8;
        let e_u16: u16 = e as u16;
        let e_u32: u32 = e as u32;
        let e_u64: u64 = e as u64;
        let e_u128: u128 = e as u128;
        let e_scalar: scalar = e as scalar;

        let f_field: field = f as field;
        let f_group: group = f as group;
        let f_bool: bool = f as bool;
        let f_i8: i8 = f as i8;
        let f_i16: i16 = f as i16;
        let f_i32: i32 = f as i32;
        let f_i64: i64 = f as i64;
        let f_i128: i128 = f as i128;
        let f_u8: u8 = f as u8;
        let f_u16: u16 = f as u16;
        let f_u32: u32 = f as u32;
        let f_u64: u64 = f as u64;
        let f_u128: u128 = f as u128;
        let f_scalar: scalar = f as scalar;

        let g_field: field = g as field;
        let g_group: group = g as group;
        let g_bool: bool = g as bool;
        let g_i8: i8 = g as i8;
        let g_i16: i16 = g as i16;
        let g_i32: i32 = g as i32;
        let g_i64: i64 = g as i64;
        let g_i128: i128 = g as i128;
        let g_u8: u8 = g as u8;
        let g_u16: u16 = g as u16;
        let g_u32: u32 = g as u32;
        let g_u64: u64 = g as u64;
        let g_u128: u128 = g as u128;
        let g_scalar: scalar = g as scalar;

        let h_field: field = h as field;
        let h_group: group = h as group;
        let h_bool: bool = h as bool;
        let h_i8: i8 = h as i8;
        let h_i16: i16 = h as i16;
        let h_i32: i32 = h as i32;
        let h_i64: i64 = h as i64;
        let h_i128: i128 = h as i128;
        let h_u8: u8 = h as u8;
        let h_u16: u16 = h as u16;
        let h_u32: u32 = h as u32;
        let h_u64: u64 = h as u64;
        let h_u128: u128 = h as u128;
        let h_scalar: scalar = h as scalar;

        let i_field: field = i as field;
        let i_group: group = i as group;
        let i_bool: bool = i as bool;
        let i_i8: i8 = i as i8;
        let i_i16: i16 = i as i16;
        let i_i32: i32 = i as i32;
        let i_i64: i64 = i as i64;
        let i_i128: i128 = i as i128;
        let i_u8: u8 = i as u8;
        let i_u16: u16 = i as u16;
        let i_u32: u32 = i as u32;
        let i_u64: u64 = i as u64;
        let i_u128: u128 = i as u128;
        let i_scalar: scalar = i as scalar;

        let j_field: field = j as field;
        let j_group: group = j as group;
        let j_bool: bool = j as bool;
        let j_i8: i8 = j as i8;
        let j_i16: i16 = j as i16;
        let j_i32: i32 = j as i32;
        let j_i64: i64 = j as i64;
        let j_i128: i128 = j as i128;
        let j_u8: u8 = j as u8;
        let j_u16: u16 = j as u16;
        let j_u32: u32 = j as u32;
        let j_u64: u64 = j as u64;
        let j_u128: u128 = j as u128;
        let j_scalar: scalar = j as scalar;

        let k_field: field = k as field;
        let k_group: group = k as group;
        let k_bool: bool = k as bool;
        let k_i8: i8 = k as i8;
        let k_i16: i16 = k as i16;
        let k_i32: i32 = k as i32;
        let k_i64: i64 = k as i64;
        let k_i128: i128 = k as i128;
        let k_u8: u8 = k as u8;
        let k_u16: u16 = k as u16;
        let k_u32: u32 = k as u32;
        let k_u64: u64 = k as u64;
        let k_u128: u128 = k as u128;

        let l_field: field = l as field;
        let l_group: group = l as group;
        let l_bool: bool = l as bool;
        let l_i8: i8 = l as i8;
        let l_i16: i16 = l as i16;
        let l_i32: i32 = l as i32;
        let l_i64: i64 = l as i64;
        let l_i128: i128 = l as i128;
        let l_u8: u8 = l as u8;
        let l_u16: u16 = l as u16;
        let l_u32: u32 = l as u32;
        let l_u64: u64 = l as u64;
        let l_u128: u128 = l as u128;
        let l_scalar: scalar = l as scalar;

        let m_field: field = m as field;
        let m_group: group = m as group;
        let m_bool: bool = m as bool;
        let m_i8: i8 = m as i8;
        let m_i16: i16 = m as i16;
        let m_i32: i32 = m as i32;
        let m_i64: i64 = m as i64;
        let m_i128: i128 = m as i128;
        let m_u8: u8 = m as u8;
        let m_u16: u16 = m as u16;
        let m_u32: u32 = m as u32;
        let m_u64: u64 = m as u64;
        let m_u128: u128 = m as u128;

        return l_field == m_field;
    }
}
