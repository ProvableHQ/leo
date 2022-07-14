// The 'message' main function.
circuit Message {
    first: field,
    second: field,
}

function main(m: Message) -> field {
    let m1: Message = Message {
        first: m.first,
        second: m.second,
    };
    return m1.first + m1.second;
}
