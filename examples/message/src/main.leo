// This example demonstrates the definition and initialization of a "circuit" type in Leo.
// Circuit types are similar to composite types in other languages such as "struct".

// The "Message" circuit type.
circuit Message {
    // A circuit member named "first" with type "field".
    first: field,
    // A circuit member named "second" with type "field".
    second: field,
}

// The "main" function of this Leo program takes a "Message" circuit type as input.
// To see how to input variable "m" is passed in open up `inputs/message.in`.
@program
function main(m: Message) -> field {

    // 1. Define the "Message" type.
    // 2. Use brackets `{ }` to enclose the circuit members.
    // 3. Define each circuit member `name : value`.
    let m1: Message = Message {
        first: m.first,
        second: m.second,
    };

    // Access the members of a circuit with dot syntax.
    // `circuit_name.member`
    return m1.first + m1.second;
}
