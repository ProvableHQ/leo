struct My__Struct {
    field__name: u8
}

fn test__function(param__name: u8) -> bool {
    return true;
}

fn triple___underscore(param___name: u8) -> bool {
    return true;
}

program test.aleo {
    fn main() -> bool {
        let test__var: u32 = 42u32;
        let trailing_double_underscore__: bool = true;

        // Leading underscores won't be lexed as an identifier.
        // Place this at the end of the test since this will terminate parsing.
        let __leading_double_underscore: bool = false;
        return true;
    }
}
