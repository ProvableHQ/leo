program test.aleo {
    storage flag: bool;
    storage scalar_val: scalar;
    storage field_val: field;
    storage group_val: group;

    fn initialize() -> Final {
        return final {
            flag = true;
            scalar_val = 1;
            field_val = 42;
            group_val = 0;
        };
    }

    fn to_none() -> Final {
        return final {
            // Integers
            flag = none;
            scalar_val = none;
            field_val = none;
            group_val = none;
        };
    }

    fn check1() -> Final {
        return final {
            assert(flag.unwrap() == true);
            assert(scalar_val.unwrap() == 1);
            assert(field_val.unwrap() == 42);
            assert(group_val.unwrap() == 0);
        };
    }

    fn check2() -> Final {
        return final {
            assert(flag.unwrap_or(false) == true);
            assert(scalar_val.unwrap_or(0) == 1);
            assert(field_val.unwrap_or(0) == 42);
            assert(group_val.unwrap_or(0) == 0);
        };
    }
}
