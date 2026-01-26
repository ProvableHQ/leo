program st.aleo {
    storage s_field: field;
    storage s_vec: [u32];
    mapping s_map: u32 => u32;

    fn assign_optional() -> Final {
        return final {
            let opt_val: field? = 5field;
            s_field = opt_val; // ❌ illegal
        };
    }

    fn assign_to_vector() -> Final {
        return final {
            s_vec = [1u32, 2u32]; // ❌ cannot assign whole vector directly
            s_vec = none;         // ❌ cannot assign none to a storage vector
        };
    }

    fn assign_vector_to_var() -> Final {
        return final {
            let local_vec1 = s_vec;       // ❌ cannot move storage vector
            let local_vec2 = [0u32; 2];
            local_vec2 = s_vec;           // ❌ cannot copy storage vector
        };
    }

    fn wrong_vector_ops() -> Final {
        return final {
            let x: field = 5field;
            x.push(3field);        // ❌ not a vector
            x.pop();               // ❌ not a vector
            x.len();               // ❌ not a vector
            x.swap_remove(0u32);   // ❌ not a vector
            x.set(0u32, 1field);   // ❌ not a vector
            x.get(0u32);           // ❌ not a vector
        };
    }

    fn wrong_vector_ops_on_storage() -> Final {
        return final {
            s_field.push(10field);     // ❌ cannot push to non-vector storage
            s_map.push(5u32);          // ❌ cannot push to mapping
        };
    }

    fn wrong_set_get() -> Final {
        return final {
            s_field.set(0u32, 10field);  // ❌ not a vector or mapping
            s_field.get(0u32);           // ❌ not a vector or mapping
        };
    }
}
