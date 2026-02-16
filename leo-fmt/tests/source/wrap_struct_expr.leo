program test.aleo {
    struct Point {
        x: u64,
        y: u64,
    }

    struct LongStruct {
        first_field: u64,
        second_field: u64,
        third_field: u64,
    }

    function main() -> u64 {
        let p: Point = Point { x: 1u64, y: 2u64 };
        let s: LongStruct = LongStruct { first_field: some_very_long_variable, second_field: another_long_variable, third_field: yet_another_long_var };
        return 0u64;
    }
}
