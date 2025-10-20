program test.aleo {
    storage flag: bool;
    storage scalar_val: scalar;
    storage field_val: field;
    storage group_val: group;

    async transition initialize() -> Future {
        return async {
            flag = true;
            scalar_val = 1;
            field_val = 42;
            group_val = 0;
        };
    }

    async transition to_none() -> Future {
        return async {
            // Integers
            flag = none;
            scalar_val = none;
            field_val = none;
            group_val = none;
        };
    }

    async transition check1() -> Future {
        return async {
            assert(flag.unwrap() == true);
            assert(scalar_val.unwrap() == 1);
            assert(field_val.unwrap() == 42);
            assert(group_val.unwrap() == 0);
        };
    }

    async transition check2() -> Future {
        return async {
            assert(flag.unwrap_or(false) == true);
            assert(scalar_val.unwrap_or(0) == 1);
            assert(field_val.unwrap_or(0) == 42);
            assert(group_val.unwrap_or(0) == 0);
        };
    }
}
