program test.aleo {
    mapping u32_map: u32 => u32;
    mapping field_map: u32 => field;
    mapping addr_map: u32 => address;

    fn update(
        s: bool,
        a_u32: u32,
        b_u32: u32,
        a_field: field,
        b_field: field,
        a_addr: address,
        b_addr: address,
    ) -> Final {
        return final {
            let chosen_u32: u32 = s ? a_u32 : b_u32;
            let chosen_field: field = s ? a_field : b_field;
            let chosen_addr: address = s ? a_addr : b_addr;
            u32_map.set(0u32, chosen_u32);
            field_map.set(0u32, chosen_field);
            addr_map.set(0u32, chosen_addr);
        };
    }
}
