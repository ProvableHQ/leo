// This example demonstrates the definition and initialization of a "struct" in Leo.
program message.aleo {
    // The "Message" struct.
    struct Message {
        // A struct member named "first" with type "field".
        first: field,
        // A struct member named "second" with type "field".
        second: field,
    }

    // The "main" function of this Leo program takes a "Message" struct type as input.
    // To see how to input variable "m" is passed in open up `inputs/message.in`.
    transition main(m: Message) -> field {

        // 1. Define the "Message" type.
        // 2. Use brackets `{ }` to enclose the struct members.
        // 3. Define each struct member `name : value`.
        let m1: Message = Message {
            first: m.first,
            second: m.second,
        };

        // Access the members of a struct with dot syntax.
        // `struct_name.member`
        return m1.first + m1.second;
    }
}
